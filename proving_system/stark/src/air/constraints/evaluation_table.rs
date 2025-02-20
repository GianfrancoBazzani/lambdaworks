use lambdaworks_math::{
    field::{element::FieldElement, traits::IsField},
    polynomial::Polynomial,
};

#[derive(Clone, Debug)]
pub struct ConstraintEvaluationTable<F: IsField> {
    // Inner vectors are rows
    pub evaluations: Vec<Vec<FieldElement<F>>>,
    pub trace_length: usize,
}

impl<F: IsField> ConstraintEvaluationTable<F> {
    pub fn new(_n_cols: usize, domain: &[FieldElement<F>]) -> Self {
        let evaluations = Vec::with_capacity(domain.len());

        ConstraintEvaluationTable {
            evaluations,
            trace_length: domain.len(),
        }
    }

    pub fn compute_composition_poly(
        &self,
        lde_coset: &[FieldElement<F>],
    ) -> Polynomial<FieldElement<F>> {
        let merged_evals: Vec<FieldElement<F>> = self
            .evaluations
            .iter()
            .map(|row| row.iter().fold(FieldElement::zero(), |acc, d| acc + d))
            .collect();

        Polynomial::interpolate(lde_coset, &merged_evals)
    }
}
