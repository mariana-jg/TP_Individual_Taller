use crate::paso_regex::PasoRegex;

pub struct PasoEvaluado {
    pub(crate) paso: PasoRegex,
    pub(crate) tam_matcheo: usize,
    pub(crate) backtrackeable: bool,
}
