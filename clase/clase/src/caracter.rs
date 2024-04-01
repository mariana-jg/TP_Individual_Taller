use crate::clase_char::ClaseChar;

#[derive(Clone, Debug)]

pub enum Caracter {
    Literal(char), 
    Wildcard,  
    Lista(ClaseChar),
    Dollar,
}

fn calcular_longitud_utf8<F>(value: &str, funcion: F) -> usize where F: Fn(char) -> bool,
{
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
            Caracter::Lista(clase) => {
                println!("en lista");
                match clase {
                    ClaseChar::Alpha => {
                        println!("en alpha");
                        if let Some(c) = value.chars().next() {
                            if c.is_alphabetic() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }
                    ClaseChar::Alnum => {
                        println!("en alnum");
                        calcular_longitud_utf8(value, char::is_alphanumeric)
                    }
                    ClaseChar::Digit => {
                        println!("en digit");
                        if let Some(c) = value.chars().next() {
                            if c.is_digit(10) {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    }
                    ClaseChar::Lower => {
                        println!("en alpha");
                        calcular_longitud_utf8(value, char::is_lowercase)
                    }
                    ClaseChar::Upper => {
                        println!("en alpha");
                        calcular_longitud_utf8(value, char::is_uppercase)
                    }
                    ClaseChar::Space => {
                        println!("en alpha");
                        calcular_longitud_utf8(value, char::is_whitespace)
                    }
                    ClaseChar::Punct => {
                        if let Some(c) = value.chars().next() {
                            if c.is_ascii_punctuation() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
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
