use crate::clase_char::ClaseChar;

#[derive(Clone, Debug)]
#[derive(PartialEq)]

pub enum Caracter {
    Literal(char), 
    Wildcard,  
    Lista(ClaseChar),
    Dollar,
}

fn calcular_longitud_utf8 <F> (value: &str, funcion: F) -> usize where F: Fn(char) -> bool, {
    if let Some(c) = value.chars().next() {
        if funcion(c) {
            c.len_utf8()
        } else {
            0
        }
    } else {
        0
    }
}

impl Caracter {
    pub fn coincide(&self, value: &str) -> usize {
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
            Caracter::Lista(clase) => {
                match clase {
                    ClaseChar::Alpha => {
                        calcular_longitud_utf8(value, char::is_alphabetic)
                    }
                    ClaseChar::Alnum => {
                        calcular_longitud_utf8(value, char::is_alphanumeric)
                    }
                    ClaseChar::Digit => {
                        calcular_longitud_utf8(value, |c| c.is_digit(10))
                    }
                    ClaseChar::Lower => {
                        calcular_longitud_utf8(value, char::is_lowercase)
                    }
                    ClaseChar::Upper => {
                        calcular_longitud_utf8(value, char::is_uppercase)
                    }
                    ClaseChar::Space => {
                        calcular_longitud_utf8(value, char::is_whitespace)
                    }
                    ClaseChar::Punct => {
                        calcular_longitud_utf8(value, |arg0: char| char::is_ascii_punctuation(&arg0))
                        /* 
                        if let Some(c) = value.chars().next() {
                            if c.is_ascii_punctuation() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }*/
                    }
                    ClaseChar::Simple(list) => {
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
            Caracter::Dollar => {
                if let Some(_) = value.chars().next() {
                    0
                } else {
                    1
                }
            }
        }
    }
}
