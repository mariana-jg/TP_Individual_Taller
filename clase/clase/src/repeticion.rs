#[derive(Clone, Debug)]

pub enum Repeticion {
    Exacta(usize, bool),
    Alguna(bool),
    Rango {
        min: Option<usize>,
        max: Option<usize>,
    },
}
