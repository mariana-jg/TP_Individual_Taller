use crate::clase_char::ClaseChar;

#[derive(Clone, Debug)]

pub enum Caracter {
    Literal(char),    //es un caracter "normal"
    Wildcard,         //coincide con cualquier caracter
   // Clase(ClaseChar), //literales, rangos y clases de caracteres
    Lista(ClaseChar),
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
                        if let Some(c) = value.chars().next() {
                            if c.is_alphanumeric() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
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
                    },
                    ClaseChar::Lower => {
                        println!("en alpha");
                        if let Some(c) = value.chars().next() {
                            if c.is_lowercase() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    ClaseChar::Upper => {
                        println!("en alpha");
                        if let Some(c) = value.chars().next() {
                            if c.is_uppercase() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    ClaseChar::Space => {
                        println!("en alpha");
                        if let Some(c) = value.chars().next() {
                            if c.is_whitespace() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
                    ClaseChar::Punct => {
                        println!("en alpha");
                        if let Some(c) = value.chars().next() {
                            if c.is_ascii_punctuation() {
                                c.len_utf8()
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    },
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
                        

                },
            }}
        }
    }
}
