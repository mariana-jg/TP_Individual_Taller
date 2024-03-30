#[derive(Clone, Debug)]

pub enum Repeticion {
    Exacta(usize, bool),
    Alguna,
    Rango {
        min: Option<usize>,
        max: Option<usize>,
    },
}
