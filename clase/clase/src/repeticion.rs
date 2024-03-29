#[derive(Clone, Debug)]

pub enum Repeticion {
    Exacta(usize),
    Alguna,
    Rango{ 
       min: Option<usize>, 
        max: Option<usize> 
    }
}