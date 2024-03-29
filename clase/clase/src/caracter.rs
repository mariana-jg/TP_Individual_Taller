use crate::clase_char::ClaseChar;

#[derive(Clone, Debug)]

pub enum Caracter {
    Literal(char), //es un caracter "normal"
    Wildcard, //coincide con cualquier caracter
    Clase(ClaseChar),
}

impl Caracter {
    pub fn coincide (&self, value: &str) -> usize {
        println!("Caracter::coincide({:?}, {:?})", self, value);
        match self {
            Caracter::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8() 
                } else {
                    0 
                }
            },
            Caracter::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8() 
                } else {
                    0 
                }
            },
            Caracter::Clase(_) => 0,
        }
    }
}