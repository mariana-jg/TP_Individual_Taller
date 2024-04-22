use crate::paso_regex::PasoRegex;
///Permite evaluar si un paso de la expresión regular se cumple o no.
///Además, se guarda la cantidad de caracteres que se matchearon y si es backtrackeable.
#[derive(Debug)]
pub struct PasoEvaluado {
    pub(crate) paso: PasoRegex,
    pub(crate) tam_matcheo: usize,
    pub(crate) backtrackeable: bool,
}
