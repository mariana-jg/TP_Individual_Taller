use crate::step_regex::StepRegex;

pub struct StepEvaluado {
    pub(crate) paso: StepRegex,
    pub(crate) match_size: usize,
    pub(crate) backtrackable: bool
}