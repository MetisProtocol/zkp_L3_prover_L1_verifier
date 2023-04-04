use crate::{
    channel::{RandomGenerator, Replayable, VerifierChannel},
    constraints::Constraints,
    polynomial::DensePolynomial,
    proof_of_work, Proof,
};
// extern crate zkp_stark;
use log::trace;
#[cfg(feature = "std")]
use std::error;
use std::{collections::BTreeMap, fmt, prelude::v1::*};
use zkp_hash::Hash;
use zkp_merkle_tree::{Commitment, Error as MerkleError, Proof as MerkleProof};
use zkp_primefield::{
    fft, geometric_series::root_series, FieldElement, One, Pow, Root, SquareInline, Zero,
};
use zkp_u256::U256;
use std::path::PathBuf;
// use structopt::StructOpt;
use ::std::os::raw::c_uint;
use ::std::os::raw::c_schar;
use ::std::os::raw::c_int;
use ::std::os::raw::c_short;
use std::convert::TryInto;
//use zkp_stark;
// use ::root::std::string;
//use structopt::StructOpt;

type Result<T> = std::result::Result<T, Error>;

// TODO - We could parametrize root unavailable with the size asked for and fri
// error with which layer failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    RootUnavailable,
    InvalidPoW,
    InvalidLDECommitment,
    InvalidConstraintCommitment,
    InvalidFriCommitment,
    HashMapFailure,
    ProofTooLong,
    OodsCalculationFailure,
    OodsMismatch,
    FriCalculationFailure,
    Merkle(MerkleError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            RootUnavailable => write!(f, "The prime field doesn't have a root of this order"),
            InvalidPoW => write!(f, "The suggested proof of work failed to verify"),
            InvalidLDECommitment => write!(f, "The LDE merkle proof is incorrect"),
            InvalidConstraintCommitment => write!(f, "The constraint merkle proof is incorrect"),
            InvalidFriCommitment => write!(f, "A FRI layer commitment is incorrect"),
            HashMapFailure => {
                write!(
                    f,
                    "Verifier attempted to look up an empty entry in the hash map"
                )
            }
            ProofTooLong => write!(f, "The proof length doesn't match the specification"),
            OodsCalculationFailure => {
                write!(
                    f,
                    "The calculated odds value doesn't match the committed one"
                )
            }
            FriCalculationFailure => {
                write!(
                    f,
                    "The final FRI calculation suggests the committed polynomial isn't low degree"
                )
            }
            OodsMismatch => write!(f, "Calculated oods value doesn't match the committed one"),
            // This is a wrapper, so defer to the underlying types' implementation of `fmt`.
            Merkle(ref e) => std::fmt::Display::fmt(e, f),
        }
    }
}

impl From<MerkleError> for Error {
    fn from(err: MerkleError) -> Self {
        Self::Merkle(err)
    }
}
/*
#[derive(StructOpt)]
struct Opt1 {
    #[structopt(name = "Assembly File")]{
    assemblyFile: string },
    #[structopt(name = "Primary Tape")]
    primaryTapeFile: string,
    #[structopt(name = "Auxillary Tape")]
    auxTapeFile: string,
    #[structopt(name = "Time Steps")]
    t: u64,
    #[structopt(name = "Security Parameter")]
    securityParameter: string,
    #[structopt(name = "Prover", parse(try_from_str))]
    prover: bool,
    #[structopt(name = "Address")]
    address: string,
    #[structopt(name = "Port Number")]
    port_number: c_uint,
    #[structopt(name = "Verbose", parse(try_from_str))]
    verbose: bool,
    #[structopt(name = "Session")]
    session: string,
    #[structopt(name = "Macros File")]
    macros_file: string

}


 #[test]
 fn testcase() {
//    let opt = Opt1::from_args();
    let security_parameter: c_int = 60;
    println!("Test Param 1");
    //crate::root::execute("examples/read_test/read_test.mips", "examples/read_test/read_test.auxtape", "examples/read_test/read_test.pubtape", 5, true, "localhost:8080", 8081,true,"10", "backend/framework/zkmetis/src/macros.json");
    unsafe {
        crate::root::execute(5, true, 8081);
    }
    println!("Test Param");
    assert_eq!(1,1);
 }
*/
// False positives on the Latex math.
#[allow(clippy::doc_markdown)]
/// # Stark verify
///
/// ## Input
///
/// A `VerifierChannel` containing the proof.
/// A `ConstraintSystem` which captures the claim that is made.
/// A `ProofParams` object which configures the proof.
///

/// <!-- TODO: ellaborate FRI verification -->
// TODO: Refactor into smaller function
#[allow(clippy::too_many_lines)]
pub fn verify(constraints: &Constraints, proof: &Proof) -> Result<()> {
    trace!("BEGIN Verify");
    let proof = proof.as_bytes();
    let trace_length = constraints.trace_nrows();
    let trace_cols = constraints.trace_ncolumns();
    let eval_domain_size = trace_length * constraints.blowup;
    let eval_x = root_series(eval_domain_size).collect::<Vec<_>>();

    let mut channel = VerifierChannel::new(proof.to_vec());
    channel.initialize(constraints.channel_seed());

    // Get the low degree root commitment, and constraint root commitment
    // TODO: Make it work as channel.read()
    let low_degree_extension_root: Hash = channel.replay();
    let lde_commitment = Commitment::from_size_hash(eval_domain_size, &low_degree_extension_root)?;
    let constraint_coefficients = channel.get_coefficients(2 * constraints.len());

    let constraint_evaluated_root: Hash = channel.replay();
    let constraint_commitment =
        Commitment::from_size_hash(eval_domain_size, &constraint_evaluated_root)?;

    // Get the oods information from the proof and random
    let oods_point: FieldElement = channel.get_random();

    // This hack is annoying and should be removed
    let mut parseable_constraints = constraints.clone();
    parseable_constraints.substitute();
    let trace_arguments = parseable_constraints.trace_arguments();
    let trace_values: Vec<FieldElement> = channel.replay_many(trace_arguments.len());
    let claimed_trace_map: BTreeMap<(usize, isize), FieldElement> = trace_arguments
        .into_iter()
        .zip(trace_values.iter().cloned())
        .collect();

    let constraints_trace_degree = constraints.degree().next_power_of_two();
    let claimed_constraint_values: Vec<FieldElement> =
        channel.replay_many(constraints_trace_degree);

    let oods_coefficients =
        channel.get_coefficients(claimed_trace_map.len() + claimed_constraint_values.len());

    let mut fri_commitments: Vec<Commitment> = Vec::with_capacity(constraints.fri_layout.len() + 1);
    let mut eval_points: Vec<FieldElement> = Vec::with_capacity(constraints.fri_layout.len() + 1);
    let mut fri_size = eval_domain_size;
    // Get fri roots and eval points from the channel random
    for &num_folds in &constraints.fri_layout {
        fri_size >>= num_folds;
        fri_commitments.push(Commitment::from_size_hash(fri_size, &channel.replay())?);
        eval_points.push(channel.get_random());
    }
    // Gets the last layer coeffiencts
    let last_layer_coefficients = channel.replay_fri_layer(fri_size / constraints.blowup);

    // Gets the proof of work from the proof.
    let pow_seed: proof_of_work::ChallengeSeed = channel.get_random();
    let pow_challenge = pow_seed.with_difficulty(constraints.pow_bits);
    let pow_response: proof_of_work::Response = channel.replay();
    if !pow_challenge.verify(pow_response) {
        return Err(Error::InvalidPoW);
    }

    // Gets queries from channel
    let queries = get_indices(
        constraints.num_queries,
        eval_domain_size.trailing_zeros(),
        &mut channel,
    );

    // Get values and check decommitment of low degree extension
    let lde_values: Vec<(usize, Vec<FieldElement>)> = queries
        .iter()
        .map(|&index| (index, channel.replay_fri_layer(trace_cols)))
        .collect();
    let lde_proof_length = lde_commitment.proof_size(&queries)?;
    let lde_hashes: Vec<Hash> = channel.replay_many(lde_proof_length);
    let lde_proof = MerkleProof::from_hashes(&lde_commitment, &queries, &lde_hashes)?;
    // Note - we could express this a merkle error instead but this adds specificity
    if lde_proof.verify(&lde_values).is_err() {
        return Err(Error::InvalidLDECommitment);
    }

    // Gets the values and checks the constraint decommitment
    let mut constraint_values = Vec::with_capacity(queries.len());
    for query_index in &queries {
        constraint_values.push((
            *query_index,
            channel.replay_fri_layer(constraints_trace_degree),
        ));
    }
    let constraint_proof_length = constraint_commitment.proof_size(&queries)?;
    let constraint_hashes: Vec<Hash> = channel.replay_many(constraint_proof_length);
    let constraint_proof =
        MerkleProof::from_hashes(&constraint_commitment, &queries, &constraint_hashes)?;
    // Note - we could express this a merkle error instead but this adds specificity
    if constraint_proof.verify(&constraint_values).is_err() {
        return Err(Error::InvalidConstraintCommitment);
    }

    let coset_sizes = constraints
        .fri_layout
        .iter()
        .map(|k| 1_usize << k)
        .collect::<Vec<_>>();
    let mut fri_indices: Vec<usize> = queries
        .to_vec()
        .iter()
        .map(|x| x / coset_sizes[0])
        .collect();

    // Folded fri values from the previous layer
    let mut fri_folds: BTreeMap<usize, FieldElement> = BTreeMap::new();

    let mut previous_indices = queries.to_vec();
    let mut step = 1;
    let mut len = eval_domain_size;
    for (k, commitment) in fri_commitments.iter().enumerate() {
        let mut fri_layer_values = Vec::new();

        fri_indices.dedup();
        for i in &fri_indices {
            let mut coset: Vec<FieldElement> = Vec::new();
            for j in 0..coset_sizes[k] {
                let n = i * coset_sizes[k] + j;
                if let Ok(z) = previous_indices.binary_search(&n) {
                    if k > 0 {
                        coset.push(match fri_folds.get(&n) {
                            Some(x) => x.clone(),
                            None => return Err(Error::HashMapFailure),
                        });
                    } else {
                        let z_reverse = fft::permute_index(eval_domain_size, queries[z]);
                        coset.push(out_of_domain_element(
                            &eval_x[z_reverse],
                            &lde_values[z].1,
                            &constraint_values[z].1,
                            &oods_point,
                            &claimed_trace_map,
                            &claimed_constraint_values,
                            &oods_coefficients,
                            trace_length,
                        )?);
                    }
                } else {
                    coset.push(channel.replay());
                }
            }
            fri_layer_values.push((*i, coset));
        }
        // Fold and record foldings
        let mut layer_folds = BTreeMap::new();
        for (i, coset) in &fri_layer_values {
            let _old_value = layer_folds.insert(
                *i,
                fri_fold(
                    coset.as_slice(),
                    &eval_points[k],
                    step,
                    (coset_sizes[k] / 2) * i,
                    len,
                    eval_x.as_slice(),
                ),
            );
        }

        let merkle_proof_length = commitment.proof_size(&fri_indices)?;
        let merkle_hashes: Vec<Hash> = channel.replay_many(merkle_proof_length);
        let merkle_proof = MerkleProof::from_hashes(commitment, &fri_indices, &merkle_hashes)?;
        fri_folds = layer_folds;

        for _ in 0..constraints.fri_layout[k] {
            step *= 2;
        }
        len /= coset_sizes[k];

        // Note - we could express this a merkle error instead but this adds specificity
        if merkle_proof.verify(&fri_layer_values).is_err() {
            return Err(Error::InvalidFriCommitment);
        };

        previous_indices = fri_indices.clone();
        if k + 1 < constraints.fri_layout.len() {
            fri_indices = fri_indices
                .iter()
                .map(|ind| ind / coset_sizes[k + 1])
                .collect();
        }
    }
    if !channel.at_end() {
        return Err(Error::ProofTooLong);
    }

    // Checks that the calculated fri folded queries are the points interpolated by
    // the decommited polynomial.
    let interp_root = match FieldElement::root(len) {
        Some(x) => x,
        None => return Err(Error::RootUnavailable),
    };
    for key in &previous_indices {
        let calculated = fri_folds[key].clone();
        let x_pow = interp_root.pow(fft::permute_index(len, *key));
        let committed = DensePolynomial::new(&last_layer_coefficients).evaluate(&x_pow);

        if committed != calculated.clone() {
            return Err(Error::OodsCalculationFailure);
        }
    }

    if oods_value_from_trace_values(
        &constraints,
        &constraint_coefficients,
        &claimed_trace_map,
        &oods_point,
    ) != oods_value_from_constraint_values(&claimed_constraint_values, &oods_point)
    {
        return Err(Error::OodsMismatch);
    }
    trace!("END Verify");
    Ok(())
}

fn oods_value_from_trace_values(
    constraints: &Constraints,
    coefficients: &[FieldElement],
    trace_values: &BTreeMap<(usize, isize), FieldElement>,
    oods_point: &FieldElement,
) -> FieldElement {
    let trace = |i: usize, j: isize| trace_values.get(&(i, j)).unwrap().clone();
    constraints
        .combine(coefficients)
        .substitute_claim(&constraints.claim_polynomials)
        .evaluate(oods_point, &trace)
}

fn oods_value_from_constraint_values(
    constraint_values: &[FieldElement],
    oods_point: &FieldElement,
) -> FieldElement {
    let mut result = FieldElement::zero();
    let mut power = FieldElement::one();
    for value in constraint_values {
        result += value * &power;
        power *= oods_point;
    }
    result
}

// TODO: Clean up
#[allow(clippy::cast_possible_truncation)]
fn get_indices(num: usize, bits: u32, proof: &mut VerifierChannel) -> Vec<usize> {
    let mut query_indices = Vec::with_capacity(num + 3);
    while query_indices.len() < num {
        let val: U256 = proof.get_random();
        query_indices
            .push(((val.clone() >> (0x100 - 0x040)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices
            .push(((val.clone() >> (0x100 - 0x080)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices
            .push(((val.clone() >> (0x100 - 0x0C0)).limb(0) & (2_u64.pow(bits) - 1)) as usize);
        query_indices.push((val.limb(0) & (2_u64.pow(bits) - 1)) as usize);
    }
    query_indices.truncate(num);
    (&mut query_indices).sort_unstable();
    query_indices
}

fn fri_fold(
    coset: &[FieldElement],
    eval_point: &FieldElement,
    mut step: usize,
    mut index: usize,
    mut len: usize,
    eval_x: &[FieldElement],
) -> FieldElement {
    let mut mutable_eval_copy = eval_point.clone();
    let mut coset_full: Vec<FieldElement> = coset.to_vec();
    while coset_full.len() > 1 {
        let mut next_coset = Vec::with_capacity(coset.len() / 2);

        for (k, pair) in coset_full.chunks(2).enumerate() {
            let x = &eval_x[fft::permute_index(len / 2, index + k) * step];
            next_coset.push(fri_single_fold(&pair[0], &pair[1], x, &mutable_eval_copy));
        }
        len /= 2;
        index /= 2;
        step *= 2;
        mutable_eval_copy = mutable_eval_copy.square();
        coset_full = next_coset;
    }
    coset_full[0].clone()
}

fn fri_single_fold(
    poly_at_x: &FieldElement,
    poly_at_neg_x: &FieldElement,
    x: &FieldElement,
    eval_point: &FieldElement,
) -> FieldElement {
    (poly_at_x + poly_at_neg_x) + eval_point / x * (poly_at_x - poly_at_neg_x)
}

#[allow(clippy::too_many_arguments)]
fn out_of_domain_element(
    query_x: &FieldElement,
    query_trace_values: &[FieldElement],
    query_constraint_values: &[FieldElement],
    oods_point: &FieldElement,
    oods_trace_map: &BTreeMap<(usize, isize), FieldElement>,
    oods_constraint_values: &[FieldElement],
    oods_coefficients: &[FieldElement],
    trace_length: usize,
) -> Result<FieldElement> {
    let shifted_x = query_x * FieldElement::generator();
    let trace_generator = match FieldElement::root(trace_length) {
        Some(x) => x,
        None => return Err(Error::RootUnavailable),
    };

    let trace_terms = oods_trace_map
        .iter()
        .map(|((column_index, offset), oods_value)| {
            (&query_trace_values[*column_index] - oods_value)
                / (&shifted_x - trace_generator.pow(*offset).unwrap() * oods_point)
        });

    let constraints_trace_degree = query_constraint_values.len();
    let combined_constraints_terms = query_constraint_values
        .iter()
        .zip(oods_constraint_values)
        .map(|(query_value, oods_value)| {
            (query_value - oods_value) / (&shifted_x - oods_point.pow(constraints_trace_degree))
        });

    Ok(trace_terms
        .chain(combined_constraints_terms)
        .zip(oods_coefficients)
        .map(|(coeffient, term)| coeffient * term)
        .sum())
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Merkle(ref e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        prove,
        traits::tests::{Recurrance, Recurrance2},
        Provable, Verifiable,
    };
    use proptest::prelude::*;
/* 
#[test]
 fn testcase() {
//    let opt = Opt1::from_args();
    let security_parameter: c_int = 60;
    println!("Test Param 1");
    //crate::root::execute("examples/read_test/read_test.mips", "examples/read_test/read_test.auxtape", "examples/read_test/read_test.pubtape", 5, true, "localhost:8080", 8081,true,"10", "backend/framewo>
    unsafe {
        crate::root::execute(5, true, 8081);
    }
    println!("Test Param");
    assert_eq!(1,1);
 }
*/
    proptest!(
        #[test]
        fn verify_recurrance(r: Recurrance) {
            let public = r.claim();
            let private = r.witness();

            let constraints = public.constraints();
            let trace = public.trace(&private);

            prop_assert!(verify(&constraints, &prove(&constraints, &trace).unwrap()).is_ok());
        }

        #[test]
        fn verify_recurrance2(r: Recurrance2) {
            let public = r.claim();
            let private = r.witness();

            let constraints = public.constraints();
            let trace = public.trace(&private);

            prop_assert!(verify(&constraints, &prove(&constraints, &trace).unwrap()).is_ok());
        }
    );
    
    #[test]
 fn testcase() {
//    let opt = Opt1::from_args();
    let security_parameter: c_int = 60;
    println!("Test Param 1");
    //crate::root::execute("examples/read_test/read_test.mips", "examples/read_test/read_test.auxtape", "examples/read_test/read_test.pubtape", 5, true, "localhost:8080", 8081,true,"10", "backend/framewo>
    unsafe {
        let flag = crate::root::execute(5, true, 1234);
        println!("Flag : {}",flag);
    }
    println!("Test Param");
    assert_eq!(1,1);
 }

}
