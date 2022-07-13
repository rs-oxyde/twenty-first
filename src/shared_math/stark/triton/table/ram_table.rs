use super::base_table::{self, BaseTable, HasBaseTable, Table};
use super::challenges_endpoints::{AllChallenges, AllEndpoints};
use super::extension_table::ExtensionTable;
use super::table_column::RamTableColumn::{self, *};
use crate::shared_math::b_field_element::BFieldElement;
use crate::shared_math::mpolynomial::MPolynomial;
use crate::shared_math::other;
use crate::shared_math::stark::triton::fri_domain::FriDomain;
use crate::shared_math::x_field_element::XFieldElement;

pub const RAM_TABLE_PERMUTATION_ARGUMENTS_COUNT: usize = 1;
pub const RAM_TABLE_EVALUATION_ARGUMENT_COUNT: usize = 0;
pub const RAM_TABLE_INITIALS_COUNT: usize =
    RAM_TABLE_PERMUTATION_ARGUMENTS_COUNT + RAM_TABLE_EVALUATION_ARGUMENT_COUNT;

/// This is 3 because it combines: clk, ramv, ramp
pub const RAM_TABLE_EXTENSION_CHALLENGE_COUNT: usize = 3;

pub const BASE_WIDTH: usize = 4;
pub const FULL_WIDTH: usize = 6; // BASE_WIDTH + 2 * INITIALS_COUNT

type BWord = BFieldElement;
type XWord = XFieldElement;

#[derive(Debug, Clone)]
pub struct RamTable {
    base: BaseTable<BWord>,
}

impl HasBaseTable<BWord> for RamTable {
    fn to_base(&self) -> &BaseTable<BWord> {
        &self.base
    }

    fn to_mut_base(&mut self) -> &mut BaseTable<BWord> {
        &mut self.base
    }
}

#[derive(Debug, Clone)]
pub struct ExtRamTable {
    base: BaseTable<XFieldElement>,
}

impl HasBaseTable<XFieldElement> for ExtRamTable {
    fn to_base(&self) -> &BaseTable<XFieldElement> {
        &self.base
    }

    fn to_mut_base(&mut self) -> &mut BaseTable<XFieldElement> {
        &mut self.base
    }
}

impl RamTable {
    pub fn new_prover(
        generator: BWord,
        order: usize,
        num_randomizers: usize,
        matrix: Vec<Vec<BWord>>,
    ) -> Self {
        let unpadded_height = matrix.len();
        let padded_height = base_table::pad_height(unpadded_height);

        let dummy = generator;
        let omicron = base_table::derive_omicron(padded_height as u64, dummy);
        let base = BaseTable::new(
            BASE_WIDTH,
            FULL_WIDTH,
            padded_height,
            num_randomizers,
            omicron,
            generator,
            order,
            matrix,
        );

        Self { base }
    }

    pub fn extend(
        &self,
        challenges: &RamTableChallenges,
        initials: &RamTableEndpoints,
    ) -> (ExtRamTable, RamTableEndpoints) {
        let mut extension_matrix: Vec<Vec<XFieldElement>> = Vec::with_capacity(self.data().len());
        let mut running_product = initials.processor_perm_product;

        for row in self.data().iter() {
            let mut extension_row = Vec::with_capacity(FULL_WIDTH);
            extension_row.extend(row.iter().map(|elem| elem.lift()));

            let (clk, ramp, ramv) = (
                extension_row[CLK as usize],
                extension_row[RAMP as usize],
                extension_row[RAMV as usize],
            );

            let (clk_w, ramp_w, ramv_w) = (
                challenges.clk_weight,
                challenges.ramp_weight,
                challenges.ramv_weight,
            );

            // 1. Compress multiple values within one row so they become one value.
            let compressed_row_for_permutation_argument =
                clk * clk_w + ramp * ramp_w + ramv * ramv_w;

            extension_row.push(compressed_row_for_permutation_argument);

            // 2. Compute the running *product* of the compressed column (permutation value)
            extension_row.push(running_product);
            running_product *=
                challenges.processor_perm_row_weight - compressed_row_for_permutation_argument;

            extension_matrix.push(extension_row);
        }

        let base = self.base.with_lifted_data(extension_matrix);
        let table = ExtRamTable { base };
        let terminals = RamTableEndpoints {
            processor_perm_product: running_product,
        };

        (table, terminals)
    }
}

impl ExtRamTable {
    pub fn with_padded_height(
        generator: XWord,
        order: usize,
        num_randomizers: usize,
        padded_height: usize,
    ) -> Self {
        let matrix: Vec<Vec<XWord>> = vec![];

        let dummy = generator;
        let omicron = base_table::derive_omicron(padded_height as u64, dummy);
        let base = BaseTable::new(
            BASE_WIDTH,
            FULL_WIDTH,
            padded_height,
            num_randomizers,
            omicron,
            generator,
            order,
            matrix,
        );

        Self { base }
    }

    pub fn ext_codeword_table(&self, fri_domain: &FriDomain<XWord>) -> Self {
        let ext_codewords = self.low_degree_extension(fri_domain, self.full_width());
        let base = self.base.with_data(ext_codewords);

        ExtRamTable { base }
    }
}

impl Table<BWord> for RamTable {
    fn name(&self) -> String {
        "RamTable".to_string()
    }

    fn pad(&mut self) {
        let data = self.mut_data();
        while !data.is_empty() && !other::is_power_of_two(data.len()) {
            let mut padding_row = data.last().unwrap().clone();
            // add same clk padding as in processor table
            padding_row[RamTableColumn::CLK as usize] = (data.len() as u32).into();
            data.push(padding_row);
        }
    }
}

impl Table<XFieldElement> for ExtRamTable {
    fn name(&self) -> String {
        "ExtRamTable".to_string()
    }

    fn pad(&mut self) {
        panic!("Extension tables don't get padded");
    }
}

impl ExtensionTable for ExtRamTable {
    fn ext_boundary_constraints(&self, _challenges: &AllChallenges) -> Vec<MPolynomial<XWord>> {
        use RamTableColumn::*;

        let variables: Vec<MPolynomial<XWord>> = MPolynomial::variables(FULL_WIDTH, 1.into());
        let clk = variables[usize::from(CLK)].clone();
        let ramp = variables[usize::from(RAMP)].clone();
        let ramv = variables[usize::from(RAMV)].clone();

        // Cycle count clk is 0.
        let clk_is_0 = clk;

        // RAM pointer ramp is 0.
        let ramp_is_0 = ramp;

        // RAM value ramv is 0.
        let ramv_is_0 = ramv;

        vec![clk_is_0, ramp_is_0, ramv_is_0]
    }

    fn ext_consistency_constraints(&self, _challenges: &AllChallenges) -> Vec<MPolynomial<XWord>> {
        // no further constraints
        vec![]
    }

    fn ext_transition_constraints(&self, _challenges: &AllChallenges) -> Vec<MPolynomial<XWord>> {
        use RamTableColumn::*;

        let variables: Vec<MPolynomial<XWord>> = MPolynomial::variables(2 * FULL_WIDTH, 1.into());
        let one = MPolynomial::from_constant(1.into(), 2 * FULL_WIDTH);

        let clk = variables[usize::from(CLK)].clone();
        let ramp = variables[usize::from(RAMP)].clone();
        let ramv = variables[usize::from(RAMV)].clone();
        let hv6 = variables[usize::from(InverseOfRampDifference)].clone();

        let clk_next = variables[FULL_WIDTH + usize::from(CLK)].clone();
        let ramp_next = variables[FULL_WIDTH + usize::from(RAMP)].clone();
        let ramv_next = variables[FULL_WIDTH + usize::from(RAMV)].clone();

        let ramp_diff = ramp_next - ramp;

        // hv6 is 0 or hv6 is the inverse of (ramp' - ramp).
        //
        // $ hv6·(hv6·(ramp' - ramp) - 1) = 0 $
        let hv6_is_0_or_hv6_is_inverse_of_ramp_diff =
            hv6.clone() * (hv6.clone() * ramp_diff.clone() - one.clone());

        // (ramp' - ramp) is zero or hv6 is the inverse of (ramp' - ramp).
        //
        // $ (ramp' - ramp)·(hv6·(ramp' - ramp) - 1) = 0 $
        let ramp_diff_is_0_or_hv6_is_inverse_of_ramp_diff =
            ramp_diff.clone() * (hv6.clone() * ramp_diff.clone() - one.clone());

        // The ramp does not change or the new ramv is 0.
        //
        // (ramp' - ramp)·ramv'
        let ramp_does_not_change_or_ramv_becomes_0 = ramp_diff.clone() * ramv_next.clone();

        // The ramp does change or the ramv does not change or the clk increases by 1.
        //
        // $ (hv6·(ramp' - ramp) - 1)·(ramv' - ramv)·(clk' - (clk + 1)) = 0 $
        let ramp_does_not_change_or_ramv_does_not_change_or_clk_increases_by_1 =
            (hv6 * ramp_diff - one.clone()) * (ramv_next - ramv) * (clk_next - (clk + one));

        vec![
            hv6_is_0_or_hv6_is_inverse_of_ramp_diff,
            ramp_diff_is_0_or_hv6_is_inverse_of_ramp_diff,
            ramp_does_not_change_or_ramv_becomes_0,
            ramp_does_not_change_or_ramv_does_not_change_or_clk_increases_by_1,
        ]
    }

    fn ext_terminal_constraints(
        &self,
        _challenges: &AllChallenges,
        _terminals: &AllEndpoints,
    ) -> Vec<MPolynomial<XWord>> {
        // no further constraints
        vec![]
    }
}

#[derive(Debug, Clone)]
pub struct RamTableChallenges {
    /// The weight that combines two consecutive rows in the
    /// permutation/evaluation column of the op-stack table.
    pub processor_perm_row_weight: XFieldElement,

    /// Weights for condensing part of a row into a single column. (Related to processor table.)
    pub clk_weight: XFieldElement,
    pub ramv_weight: XFieldElement,
    pub ramp_weight: XFieldElement,
}

#[derive(Debug, Clone)]
pub struct RamTableEndpoints {
    /// Values randomly generated by the prover for zero-knowledge.
    pub processor_perm_product: XFieldElement,
}
