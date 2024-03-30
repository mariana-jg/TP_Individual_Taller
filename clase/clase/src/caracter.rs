use crate::clase_char::ClaseChar;

#[derive(Clone, Debug)]

pub enum Caracter {
    Literal(char),    //es un caracter "normal"
    Wildcard,         //coincide con cualquier caracter
    Clase(ClaseChar), //literales, rangos y clases de caracteres
    Lista(Vec<char>),
    //2 tipos de literales: los que son caracteres literales y los que son rangos
}

impl Caracter {
    pub fn coincide(&self, value: &str) -> usize {
        println!("Caracter::coincide({:?}, {:?})", self, value);
        match self {
            Caracter::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8()
                } else {
                    0
                }
            }
            Caracter::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8()
                } else {
                    0
                }
            }
            Caracter::Clase(_) => 0,
            Caracter::Lista(list) => {
                if let Some(c) = value.chars().next() {
                    if list.contains(&c) {
                        c.len_utf8()
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        }
    }
}
