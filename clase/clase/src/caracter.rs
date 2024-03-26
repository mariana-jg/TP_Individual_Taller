use crate::clase_char::ClaseChar;

#[derive(Clone)]

pub enum Caracter {
    Literal(char), //se que tengo el valor con el que quiero matchear
    Wildcard, //comodin
    Clase(ClaseChar),
}

impl Caracter {
    pub fn coincide (&self, value: &str) -> usize {
        //deberia devolver usize ya que quisiera devolver
        //cuanto coinsumi del input cuando matchee
        match self {
            Caracter::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8() //cantidad consumida en el input
                } else {
                    0 
                }
            },
            Caracter::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8() //cantidad consumida en el input
                } else {
                    0 
                }
            },
            Caracter::Clase(_) => 0,
        }
    }
}