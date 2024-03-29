#[derive(Debug, PartialEq)]
pub enum ResultadoValidacion {
    Encontrado { avance: usize },
    NoEncontrado { avance: usize },
    //LineaTerminada
}