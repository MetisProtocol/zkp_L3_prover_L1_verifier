#![cfg(target_arch = "aarch64")]

use crate::allocator::PageAlignedAllocator;
use crate::stage::BitReverseGpuStage;
use crate::stage::FftGpuStage;
use crate::stage::ScaleAndNormalizeGpuStage;
use crate::stage::Variant;
use crate::utils;
use crate::GpuField;
use crate::GpuVec;
use alloc::rc::Rc;
use alloc::vec::Vec;
use ark_ff::One;
use ark_ff::Zero;
use ark_poly::EvaluationDomain;
use ark_poly::Radix2EvaluationDomain;
use once_cell::sync::Lazy;

const LIBRARY_DATA: &[u8] = include_bytes!("metal/shaders.metallib");

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum FftDirection {
    /// FFT
    Forward,
    /// IFFT
    Inverse,
}

pub struct FftEncoder<'a, F: GpuField> {
    n: usize,
    command_queue: Rc<metal::CommandQueue>,
    // twiddles_buffer references this memory
    // field exists to keep the memory around
    _twiddles: GpuVec<F::FftField>,
    twiddles_buffer: metal::Buffer,
    scale_and_normalize_stage: Option<ScaleAndNormalizeGpuStage<F>>,
    butterfly_stages: Vec<FftGpuStage<F>>,
    bit_reverse_stage: BitReverseGpuStage<F>,
    command_buffer: &'a metal::CommandBufferRef,
}

// // https://github.com/gfx-rs/metal-rs/issues/40
// unsafe impl<'a, F: GpuField> Send for FftEncoder<'a, F> {}
// unsafe impl<'a, F: GpuField> Sync for FftEncoder<'a, F> {}

impl<'a, F: GpuField> FftEncoder<'a, F> {
    fn encode_butterfly_stages(&self, input_buffer: &mut metal::Buffer) {
        for stage in &self.butterfly_stages {
            stage.encode(self.command_buffer, input_buffer, &self.twiddles_buffer);
        }
    }

    fn encode_bit_reverse_stage(&self, input_buffer: &mut metal::Buffer) {
        self.bit_reverse_stage
            .encode(self.command_buffer, input_buffer);
    }

    fn encode_scale_stage(&self, input_buffer: &mut metal::Buffer) {
        if let Some(scale_stage) = &self.scale_and_normalize_stage {
            scale_stage.encode(self.command_buffer, input_buffer);
        }
    }

    // TODO: change to &mut
    pub fn execute(self) {
        self.command_buffer.commit();
        self.command_buffer.wait_until_completed();
    }
}

pub struct GpuFft<'a, F: GpuField>(FftEncoder<'a, F>);

impl<'a, F: GpuField> GpuFft<'a, F> {
    pub const MIN_SIZE: usize = 2048;

    pub fn encode(&mut self, buffer: &mut GpuVec<F>) {
        assert!(self.0.n >= buffer.len());
        buffer.resize(self.0.n, F::zero());
        let mut input_buffer = utils::buffer_mut_no_copy(self.0.command_queue.device(), buffer);
        self.0.encode_scale_stage(&mut input_buffer);
        self.0.encode_butterfly_stages(&mut input_buffer);
        self.0.encode_bit_reverse_stage(&mut input_buffer);
    }

    pub fn execute(self) {
        self.0.execute()
    }
}

impl<'a, F: GpuField> From<Radix2EvaluationDomain<F::FftField>> for GpuFft<'a, F> {
    fn from(domain: Radix2EvaluationDomain<F::FftField>) -> Self {
        PLANNER.plan_fft(domain)
    }
}

pub struct GpuIfft<'a, F: GpuField>(FftEncoder<'a, F>);

impl<'a, F: GpuField> GpuIfft<'a, F> {
    pub const MIN_SIZE: usize = 2048;

    pub fn encode(&mut self, input: &mut GpuVec<F>) {
        assert_eq!(self.0.n, input.len());
        let mut input_buffer = utils::buffer_mut_no_copy(self.0.command_queue.device(), input);
        self.0.encode_butterfly_stages(&mut input_buffer);
        self.0.encode_bit_reverse_stage(&mut input_buffer);
        self.0.encode_scale_stage(&mut input_buffer);
    }

    pub fn execute(self) {
        self.0.execute()
    }
}

impl<'a, F: GpuField> From<Radix2EvaluationDomain<F::FftField>> for GpuIfft<'a, F> {
    fn from(domain: Radix2EvaluationDomain<F::FftField>) -> Self {
        PLANNER.plan_ifft(domain)
    }
}

pub static PLANNER: Lazy<Planner> = Lazy::new(Planner::default);

pub struct Planner {
    pub library: metal::Library,
    pub command_queue: Rc<metal::CommandQueue>,
}

// TODO: unsafe
unsafe impl Send for Planner {}
unsafe impl Sync for Planner {}

impl Planner {
    pub fn new(device: &metal::DeviceRef) -> Self {
        let library = device.new_library_with_data(LIBRARY_DATA).unwrap();
        let command_queue = Rc::new(device.new_command_queue());
        Self {
            library,
            command_queue,
        }
    }

    pub fn plan_fft<F: GpuField>(&self, domain: Radix2EvaluationDomain<F::FftField>) -> GpuFft<F> {
        assert!(domain.size() >= GpuFft::<F>::MIN_SIZE);
        GpuFft(self.create_fft_encoder(FftDirection::Forward, domain))
    }

    pub fn plan_ifft<F: GpuField>(
        &self,
        domain: Radix2EvaluationDomain<F::FftField>,
    ) -> GpuIfft<F> {
        assert!(domain.size() >= GpuIfft::<F>::MIN_SIZE);
        GpuIfft(self.create_fft_encoder(FftDirection::Inverse, domain))
    }

    // TODO: move to FftEncoder struct
    fn create_fft_encoder<F: GpuField>(
        &self,
        direction: FftDirection,
        domain: Radix2EvaluationDomain<F::FftField>,
    ) -> FftEncoder<F> {
        let n = domain.size();
        let device = self.command_queue.device();

        let root = match direction {
            FftDirection::Forward => domain.group_gen,
            FftDirection::Inverse => domain.group_gen_inv,
        };

        // generate twiddles buffer
        let mut _twiddles = Vec::with_capacity_in(n / 2, PageAlignedAllocator);
        _twiddles.resize(n / 2, F::FftField::zero());
        utils::fill_twiddles(&mut _twiddles, root);
        utils::bit_reverse(&mut _twiddles);
        let twiddles_buffer = utils::buffer_no_copy(device, &_twiddles);

        // in-place FFT requires a bit reversal
        let bit_reverse_stage = BitReverseGpuStage::new(&self.library, n);

        // scale and normalise
        let scale_and_normalize_stage = if direction == FftDirection::Forward {
            if domain.offset.is_one() {
                None
            } else {
                Some(ScaleAndNormalizeGpuStage::new(
                    &self.library,
                    &self.command_queue,
                    n,
                    domain.offset.into(),
                    F::one(),
                ))
            }
        } else {
            Some(ScaleAndNormalizeGpuStage::new(
                &self.library,
                &self.command_queue,
                n,
                domain.offset_inv.into(),
                domain.size_inv.into(),
            ))
        };

        // stages that involve an FFT butterfly
        let mut butterfly_stages = Vec::new();
        let threadgroup_mem_len = device.max_threadgroup_memory_length() as usize;
        // TODO: get max_threads_per_threadgroup from metal api. Depends on pipeline
        let threadgroup_fft_size = utils::threadgroup_fft_size::<F>(threadgroup_mem_len, 1024);
        for stage in 0..n.ilog2() {
            let variant = if n >> stage == threadgroup_fft_size {
                Variant::Multiple
            } else {
                Variant::Single
            };

            butterfly_stages.push(FftGpuStage::new(
                &self.library,
                n,
                1 << stage,
                variant,
                threadgroup_fft_size,
            ));

            if let Variant::Multiple = variant {
                break;
            }
        }

        FftEncoder {
            n,
            _twiddles,
            twiddles_buffer,
            scale_and_normalize_stage,
            butterfly_stages,
            bit_reverse_stage,
            command_queue: Rc::clone(&self.command_queue),
            command_buffer: self.command_queue.new_command_buffer(),
        }
    }
}

impl Default for Planner {
    fn default() -> Self {
        Planner::new(&metal::Device::system_default().expect("no device found"))
    }
}
