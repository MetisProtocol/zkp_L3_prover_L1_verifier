use crate::air::MipsAir;
use crate::air::ExecutionInfo;
use crate::trace::MipsTrace;
use gpu_poly::fields::p18446744069414584321::Fp;
use gpu_poly::fields::p18446744069414584321::Fq3;
use ministark::ProofOptions;
use ministark::Prover;

pub struct MipsProver(ProofOptions);

impl Prover for MipsProver {
    type Fp = Fp;
    type Fq = Fq3;
    type Air = MipsAir;
    type Trace = MipsTrace;

    fn new(options: ProofOptions) -> Self {
        MipsProver(options)
    }

    fn options(&self) -> ProofOptions {
        self.0
    }

    fn get_pub_inputs(&self, trace: &MipsTrace) -> ExecutionInfo {
        let meta = trace.meta();
        ExecutionInfo {
            source_code: meta.source_code.to_string(),
            input: meta.input.to_vec(),
            output: meta.output.to_vec(),
        }
    }
}
