use super::vm::{InstructionMatrixBaseRow, Register};
use crate::shared_math::mpolynomial::Degree;
use crate::shared_math::other::roundup_npo2;
use crate::shared_math::polynomial::Polynomial;
use crate::shared_math::stark::brainfuck::evaluation_argument::{
    EvaluationArgument, ProgramEvaluationArgument,
};
use crate::shared_math::stark::brainfuck::instruction_table::InstructionTable;
use crate::shared_math::stark::brainfuck::io_table::IOTable;
use crate::shared_math::stark::brainfuck::memory_table::MemoryTable;
use crate::shared_math::stark::brainfuck::permutation_argument::PermutationArgument;
use crate::shared_math::stark::brainfuck::table;
use crate::shared_math::stark::brainfuck::table_collection::TableCollection;
use crate::shared_math::traits::{FromVecu8, GetRandomElements, ModPowU32};
use crate::shared_math::{
    b_field_element::BFieldElement, fri::Fri, other::is_power_of_two,
    stark::brainfuck::processor_table::ProcessorTable, traits::GetPrimitiveRootOfUnity,
    x_field_element::XFieldElement,
};
use crate::util_types::merkle_tree::MerkleTree;
use crate::util_types::proof_stream::ProofStream;
use crate::util_types::simple_hasher::{Hasher, RescuePrimeProduction};
use itertools::Itertools;
use rand::thread_rng;
use std::cell::RefCell;
use std::convert::TryInto;
use std::error::Error;
use std::rc::Rc;

pub const EXTENSION_CHALLENGE_COUNT: usize = 11;
pub const PERMUTATION_ARGUMENTS_COUNT: usize = 2;
pub const TERMINAL_COUNT: usize = 5;

pub struct Stark {
    trace_length: usize,
    program: Vec<BFieldElement>,
    input_symbols: Vec<BFieldElement>,
    output_symbols: Vec<BFieldElement>,
    expansion_factor: u64,
    security_level: usize,
    colinearity_checks_count: usize,
    num_randomizers: usize,
    base_tables: Rc<RefCell<TableCollection>>,
    max_degree: u64,
    fri: Fri<BFieldElement, blake3::Hasher>,

    permutation_arguments: [PermutationArgument; PERMUTATION_ARGUMENTS_COUNT],
    io_evaluation_arguments: [EvaluationArgument; 2],
    program_evaluation_argument: ProgramEvaluationArgument,
}

impl Stark {
    // TODO: Change this to use Rescue prime instead of Vec<u8>/Blake3
    // TODO: Use simple_hasher's get_n_hash_rounds() instead.
    fn sample_weights(number: u8, seed: Vec<u8>) -> Vec<XFieldElement> {
        let mut challenges: Vec<XFieldElement> = vec![];
        for i in 0..number {
            let mut mutated_challenge_seed = seed.clone();
            mutated_challenge_seed[0] = ((mutated_challenge_seed[0] as u16 + i as u16) % 256) as u8;
            // This is wrong:
            challenges.push(XFieldElement::ring_zero().from_vecu8(mutated_challenge_seed));
        }

        challenges
    }

    pub fn new(
        trace_length: usize,
        program: Vec<BFieldElement>,
        input_symbols: Vec<BFieldElement>,
        output_symbols: Vec<BFieldElement>,
    ) -> Self {
        let log_expansion_factor = 2; // TODO: For speed
        let expansion_factor: u64 = 1 << log_expansion_factor;
        let security_level = 2; // TODO: Consider increasing this
        let colinearity_checks_count = security_level / log_expansion_factor;
        assert!(
            colinearity_checks_count > 0,
            "At least one colinearity check is required"
        );
        assert!(
            is_power_of_two(expansion_factor),
            "expansion factor must be power of two."
        );
        assert!(
            expansion_factor >= 4,
            "expansion factor must be at least 4."
        );

        let num_randomizers = 1;
        let order: usize = 1 << 32;
        let smooth_generator = BFieldElement::ring_zero()
            .get_primitive_root_of_unity(order as u128)
            .0
            .unwrap();

        // instantiate table objects
        let processor_table =
            ProcessorTable::new(trace_length, num_randomizers, smooth_generator, order);

        let instruction_table = InstructionTable::new(
            trace_length + program.len(),
            num_randomizers,
            smooth_generator,
            order,
        );

        let memory_table = MemoryTable::new(trace_length, num_randomizers, smooth_generator, order);
        let input_table = IOTable::new_input_table(input_symbols.len(), smooth_generator, order);
        let output_table = IOTable::new_output_table(output_symbols.len(), smooth_generator, order);

        let base_tables = TableCollection::new(
            processor_table,
            instruction_table,
            memory_table,
            input_table,
            output_table,
        );

        // instantiate permutation objects
        let rc_base_tables = Rc::new(RefCell::new(base_tables));

        let processor_instruction_lhs = (
            table::PROCESSOR_TABLE,
            ProcessorTable::INSTRUCTION_PERMUTATION,
        );
        let processor_instruction_rhs = (table::INSTRUCTION_TABLE, InstructionTable::PERMUTATION);
        let processor_instruction_permutation = PermutationArgument::new(
            rc_base_tables.clone(),
            processor_instruction_lhs,
            processor_instruction_rhs,
        );

        let processor_memory_lhs = (table::PROCESSOR_TABLE, ProcessorTable::MEMORY_PERMUTATION);
        let processor_memory_rhs = (table::MEMORY_TABLE, MemoryTable::PERMUTATION);
        let processor_memory_permutation = PermutationArgument::new(
            rc_base_tables.clone(),
            processor_memory_lhs,
            processor_memory_rhs,
        );

        let permutation_arguments: [PermutationArgument; 2] = [
            processor_instruction_permutation,
            processor_memory_permutation,
        ];

        // input_evaluation = EvaluationArgument(
        //     8, 2, [BaseFieldElement(ord(i), self.field) for i in input_symbols])
        let input_evaluation = EvaluationArgument::new(
            rc_base_tables.borrow().input_table.challenge_index(),
            rc_base_tables.borrow().input_table.terminal_index(),
            input_symbols.clone(),
        );

        // output_evaluation = EvaluationArgument(
        //     9, 3, [BaseFieldElement(ord(o), self.field) for o in output_symbols])
        let output_evaluation = EvaluationArgument::new(
            rc_base_tables.borrow().output_table.challenge_index(),
            rc_base_tables.borrow().output_table.terminal_index(),
            output_symbols.clone(),
        );
        let io_evaluation_arguments = [input_evaluation, output_evaluation];

        // program_evaluation = ProgramEvaluationArgument(
        //     [0, 1, 2, 6], 4, program)
        let program_challenge_indices = vec![0, 1, 2, 6];
        let program_terminal_index = 4;

        let program_evaluation_argument = ProgramEvaluationArgument::new(
            program_challenge_indices,
            program_terminal_index,
            program.clone(),
        );

        // Compute max degree
        let mut max_degree: u64 = rc_base_tables.borrow().get_max_degree();
        max_degree = roundup_npo2(max_degree) - 1;
        let fri_domain_length: u64 = (max_degree + 1) * expansion_factor;

        // Instantiate FRI object
        let b_field_generator = BFieldElement::generator();
        let b_field_omega = BFieldElement::ring_zero()
            .get_primitive_root_of_unity(fri_domain_length as u128)
            .0
            .unwrap();
        let fri: Fri<BFieldElement, blake3::Hasher> = Fri::new(
            b_field_generator,
            b_field_omega,
            fri_domain_length as usize,
            expansion_factor as usize,
            colinearity_checks_count,
        );

        Self {
            trace_length,
            program,
            input_symbols,
            output_symbols,
            expansion_factor,
            security_level,
            colinearity_checks_count,
            num_randomizers,
            base_tables: rc_base_tables,
            max_degree,
            fri,
            permutation_arguments,
            io_evaluation_arguments,
            program_evaluation_argument,
        }
    }

    // def prove(self, running_time, program, processor_matrix, instruction_matrix, input_matrix, output_matrix, proof_stream=None):

    pub fn prove(
        &mut self,
        processor_matrix: Vec<Register>,
        instruction_matrix: Vec<InstructionMatrixBaseRow>,
        input_matrix: Vec<BFieldElement>,
        output_matrix: Vec<BFieldElement>,
    ) -> Result<ProofStream, Box<dyn Error>> {
        assert_eq!(self.trace_length, processor_matrix.len());
        assert_eq!(
            self.trace_length + self.program.len(),
            instruction_matrix.len(),
            "instruction_matrix must contain both the execution trace and the program"
        );

        self.base_tables.borrow_mut().set_matrices(
            processor_matrix,
            instruction_matrix,
            input_matrix,
            output_matrix,
        );

        self.base_tables.borrow_mut().pad();

        // Instantiate the memory table object
        let processor_matrix_clone = self.base_tables.borrow().processor_table.0.matrix.clone();
        self.base_tables.borrow_mut().memory_table.0.matrix =
            MemoryTable::derive_matrix(processor_matrix_clone);

        // Generate randomizer codewords for zero-knowledge
        // This generates three B field randomizer codewords, each with the same length as the FRI domain
        let mut rng = thread_rng();
        let randomizer_polynomial = Polynomial::new(XFieldElement::random_elements(
            self.max_degree as usize + 1,
            &mut rng,
        ));
        let x_randomizer_codeword: Vec<XFieldElement> =
            self.fri.domain.xevaluate(&randomizer_polynomial);
        let mut b_randomizer_codewords: [Vec<BFieldElement>; 3] = [vec![], vec![], vec![]];
        for x_elem in x_randomizer_codeword.iter() {
            b_randomizer_codewords[0].push(x_elem.coefficients[0]);
            b_randomizer_codewords[1].push(x_elem.coefficients[1]);
            b_randomizer_codewords[2].push(x_elem.coefficients[2]);
        }

        let base_codewords: Vec<Vec<BFieldElement>> = self
            .base_tables
            .borrow_mut()
            .get_and_set_all_base_codewords(&self.fri.domain);
        let all_base_codewords =
            vec![base_codewords.clone(), b_randomizer_codewords.into()].concat();

        let _base_degree_bounds = self.base_tables.borrow().get_all_base_degree_bounds();

        // TODO: How do I make a single Merkle tree from many codewords?
        // If the Merkle trees are always opened for all base codewords for a single index, then
        // we *should* be able to make a commitment to *each* index and store that list of commitments
        // in a single Merkle tree. This list of commitments will have length 2^k, so this should be
        // possible, as the MT requires a leaf count that is a power of two.
        let transposed_base_codewords: Vec<Vec<BFieldElement>> = (0..all_base_codewords[0].len())
            .map(|i| {
                all_base_codewords
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<BFieldElement>>()
            })
            .collect();
        let mut hasher = RescuePrimeProduction::new();

        // Current length of each element in `transposed_base_codewords` is 18 which exceeds
        // max length of RP hash function. So we chop it into elements that will fit into the
        // rescue prime hash function. This is done by chopping the hash function input into
        // chunks of `max_length / 2` and calling `hash_many` on this input. Half the max
        // length is needed since the chunks are hashed two at a time.
        let base_codeword_digests_by_index: Vec<Vec<BFieldElement>> = transposed_base_codewords
            .clone()
            .into_iter()
            .map(|values| {
                let chunks: Vec<Vec<BFieldElement>> = values
                    .chunks(hasher.0.max_input_length / 2)
                    .map(|s| s.into())
                    .collect();
                hasher.hash_many(&chunks)
            })
            .collect();
        let base_merkle_tree =
            MerkleTree::<Vec<BFieldElement>, RescuePrimeProduction>::from_digests(
                &base_codeword_digests_by_index,
                &vec![BFieldElement::ring_zero()],
            );

        // Commit to base codewords
        let mut proof_stream = ProofStream::default();
        proof_stream.enqueue(base_merkle_tree.get_root())?;

        // Get coefficients for table extension
        // let challenges = self.sample_weights(EXTENSION_CHALLENGE_COUNT, proof_stream.prover_fiat_shamir());
        // TODO: REPLACE THIS WITH RescuePrime/B field elements. The type of `challenges`
        // must not change though, it should remain `Vec<XFieldElement>`.
        let challenges: [XFieldElement; EXTENSION_CHALLENGE_COUNT] = Self::sample_weights(
            EXTENSION_CHALLENGE_COUNT as u8,
            proof_stream.prover_fiat_shamir(),
        )
        .try_into()
        .unwrap();

        let initials: [XFieldElement; PERMUTATION_ARGUMENTS_COUNT] =
            XFieldElement::random_elements(PERMUTATION_ARGUMENTS_COUNT, &mut rng)
                .try_into()
                .unwrap();

        self.base_tables.borrow_mut().extend(challenges, initials);

        let extension_codewords = self
            .base_tables
            .borrow_mut()
            .get_and_set_all_extension_codewords(&self.fri.domain);

        let transposed_extension_codewords: Vec<Vec<XFieldElement>> = (0..extension_codewords[0]
            .len())
            .map(|i| {
                extension_codewords
                    .iter()
                    .map(|inner| inner[i].clone())
                    .collect::<Vec<XFieldElement>>()
            })
            .collect();
        let extension_codeword_digests_by_index: Vec<Vec<BFieldElement>> =
            transposed_extension_codewords
                .clone()
                .into_iter()
                .map(|xvalues| {
                    let bvalues: Vec<BFieldElement> = xvalues
                        .into_iter()
                        .map(|x| x.coefficients.clone().to_vec())
                        .concat();
                    assert_eq!(
                        27,
                        bvalues.len(),
                        "9 X-field elements must become 27 B-field elements"
                    );
                    let chunks: Vec<Vec<BFieldElement>> = bvalues
                        .chunks(hasher.0.max_input_length / 2)
                        .map(|s| s.into())
                        .collect();
                    assert_eq!(
                        6,
                        chunks.len(),
                        "27 B-field elements must be divided into 6 chunks for hashing"
                    );
                    hasher.hash_many(&chunks)
                })
                .collect();
        let extension_tree = MerkleTree::<Vec<BFieldElement>, RescuePrimeProduction>::from_digests(
            &extension_codeword_digests_by_index,
            &vec![BFieldElement::ring_zero()],
        );
        proof_stream.enqueue(extension_tree.get_root())?;

        let extension_degree_bounds: Vec<Degree> =
            self.base_tables.borrow().get_all_extension_degree_bounds();

        let terminals: [XFieldElement; TERMINAL_COUNT] = self.base_tables.borrow().get_terminals();

        let mut quotient_codewords =
            self.base_tables
                .borrow()
                .all_quotients(&self.fri.domain, challenges, terminals);
        let mut quotient_degree_bounds = self
            .base_tables
            .borrow()
            .all_quotient_degree_bounds(challenges, terminals);

        for pa in self.permutation_arguments.iter() {
            quotient_codewords.push(pa.quotient(&self.fri.domain));
            quotient_degree_bounds.push(pa.quotient_degree_bound());
        }

        for t in terminals.iter() {
            proof_stream.enqueue(t)?;
        }

        // TODO: Get weights for nonlinear combination
        let num_base_polynomials: usize = self
            .base_tables
            .borrow()
            .into_iter()
            .map(|table| table.base_width())
            .sum();
        let num_extension_polynomials: usize = self
            .base_tables
            .borrow()
            .into_iter()
            .map(|table| table.full_width() - table.base_width())
            .sum();
        let num_randomizer_polynomials: usize = 1;
        let num_quotient_polynomials: usize = quotient_degree_bounds.len();
        let weights_seed = proof_stream.prover_fiat_shamir();
        let weights = Self::sample_weights(
            (num_randomizer_polynomials
                + 2 * (num_base_polynomials + num_extension_polynomials + num_quotient_polynomials))
                as u8,
            weights_seed,
        );

        let mut terms: Vec<Vec<XFieldElement>> = vec![x_randomizer_codeword];
        assert_eq!(base_codewords.len(), num_base_polynomials);
        let fri_x_values: Vec<BFieldElement> = self.fri.domain.x_values();
        for (i, (bc, bdb)) in base_codewords
            .iter()
            .zip(_base_degree_bounds.iter())
            .enumerate()
        {
            let bc_lifted: Vec<XFieldElement> = bc.iter().map(|bfe| bfe.lift()).collect();
            terms.push(bc_lifted);
            let shift = (self.max_degree as Degree - bdb) as u32;
            let bc_shifted: Vec<XFieldElement> = fri_x_values
                .iter()
                .zip(bc.iter())
                .map(|(x, &bce)| (bce * x.mod_pow_u32(shift)).lift())
                .collect();
            terms.push(bc_shifted);

            if std::env::var("DEBUG").is_ok() {
                let interpolated = self.fri.domain.xinterpolate(terms.last().unwrap());
                assert!(
                    interpolated.degree() == -1
                        || interpolated.degree() == self.max_degree as isize,
                    "The shifted base codeword with index {} must be of maximal degree {}. Got {}.",
                    i,
                    self.max_degree,
                    interpolated.degree()
                );
            }
        }

        assert_eq!(extension_codewords.len(), num_extension_polynomials);
        for (i, (ec, edb)) in extension_codewords
            .iter()
            .zip(extension_degree_bounds.iter())
            .enumerate()
        {
            terms.push(ec.to_vec());
            let shift = (self.max_degree as Degree - edb) as u32;
            let ec_shifted: Vec<XFieldElement> = fri_x_values
                .iter()
                .zip(ec.iter())
                .map(|(x, &ece)| ece * x.mod_pow_u32(shift).lift())
                .collect();
            terms.push(ec_shifted);

            if std::env::var("DEBUG").is_ok() {
                let interpolated = self.fri.domain.xinterpolate(terms.last().unwrap());
                assert!(
                    interpolated.degree() == -1
                        || interpolated.degree() == self.max_degree as isize,
                    "The shifted extension codeword with index {} must be of maximal degree {}. Got {}.",
                    i,
                    self.max_degree,
                    interpolated.degree()
                );
            }
        }

        // assert_eq!(quotient_codewords.len(), num_quotient_polynomials);
        // for (i, (qc, qdb)) in quotient_codewords
        //     .iter()
        //     .zip(quotient_degree_bounds.iter())
        //     .enumerate()
        // {
        //     terms.push(qc.to_vec());
        //     let shift = (self.max_degree as Degree - qdb) as u32;
        //     let qc_shifted: Vec<XFieldElement> = fri_x_values
        //         .iter()
        //         .zip(qc.iter())
        //         .map(|(x, &qce)| qce * x.mod_pow_u32(shift).lift())
        //         .collect();
        //     terms.push(qc_shifted);

        //     if std::env::var("DEBUG").is_ok() {
        //         let interpolated = self.fri.domain.xinterpolate(terms.last().unwrap());
        //         assert!(
        //             interpolated.degree() == -1
        //                 || interpolated.degree() == self.max_degree as isize,
        //             "The shifted quotient codeword with index {} must be of maximal degree {}. Got {}.",
        //             i,
        //             self.max_degree,
        //             interpolated.degree()
        //         );
        //     }
        // }

        // // Take weighted sum
        // assert_eq!(terms.len(), weights.len());

        Ok(proof_stream)
    }
}

#[cfg(test)]
mod brainfuck_stark_tests {
    use super::*;
    use crate::shared_math::b_field_element::BFieldElement;
    use crate::shared_math::stark::brainfuck;
    use crate::shared_math::stark::brainfuck::vm::BaseMatrices;

    #[test]
    fn prove_verify_test() {
        let program: Vec<BFieldElement> =
            brainfuck::vm::compile(brainfuck::vm::sample_programs::VERY_SIMPLE_PROGRAM).unwrap();
        let (trace_length, input_symbols, output_symbols) =
            brainfuck::vm::run(&program, vec![]).unwrap();
        let base_matrices: BaseMatrices =
            brainfuck::vm::simulate(&program, &input_symbols).unwrap();
        let mut stark = Stark::new(trace_length, program, input_symbols, output_symbols);

        // TODO: If we set the `DEBUG` environment variable here, we *should* catch a lot of bugs.
        // Do we want to do that?
        let _proof_stream = stark
            .prove(
                base_matrices.processor_matrix,
                base_matrices.instruction_matrix,
                base_matrices.input_matrix,
                base_matrices.output_matrix,
            )
            .unwrap();
    }
}
