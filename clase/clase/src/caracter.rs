use crate::clase_char::ClaseChar;

#[derive(Clone, Debug, PartialEq)]

pub enum Caracter {
    Literal(char),
    Comodin,
    Serie(ClaseChar),
    Dolar,
}

fn calcular_longitud_utf8_clase<F>(valor: &str, funcion: F) -> usize
where
    F: Fn(char) -> bool,
{
    if let Some(c) = valor.chars().next() {
        if funcion(c) {
            c.len_utf8()
        } else {
            0
        }
    } else {
        0
    }
}

fn calcular_longitud_utf8_literal(valor: &str, l: &char) -> usize {
    if valor.starts_with(*l) {
        l.len_utf8()
    } else {
        0
    }
}

fn calcular_longitud_utf8_comodin(valor: &str) -> usize {
    if let Some(c) = valor.chars().next() {
        c.len_utf8()
    } else {
        0
    }
}

fn calcular_longitud_utf8_dolar(valor: &str) -> usize {
    if valor.chars().next().is_some() {
        0
    } else {
        1
    }
}

impl Caracter {
    pub fn coincide(&self, valor: &str) -> usize {
        match self {
            Caracter::Literal(l) => calcular_longitud_utf8_literal(valor, l),
            Caracter::Comodin => calcular_longitud_utf8_comodin(valor),
            Caracter::Serie(clase) => match clase {
                ClaseChar::Alpha => {
                    calcular_longitud_utf8_clase(valor, |c: char| char::is_ascii_alphabetic(&c))
                }
                ClaseChar::Alnum => calcular_longitud_utf8_clase(valor, char::is_alphanumeric),
                ClaseChar::Digit => calcular_longitud_utf8_clase(valor, |c| c.is_ascii_digit()),
                ClaseChar::Lower => calcular_longitud_utf8_clase(valor, char::is_lowercase),
                ClaseChar::Upper => calcular_longitud_utf8_clase(valor, char::is_uppercase),
                ClaseChar::Space => calcular_longitud_utf8_clase(valor, char::is_whitespace),
                ClaseChar::Punct => {
                    calcular_longitud_utf8_clase(valor, |c: char| char::is_ascii_punctuation(&c))
                }
                ClaseChar::Simple(list) => {
                    calcular_longitud_utf8_clase(valor, |c| list.contains(&c))
                }
            },
            Caracter::Dolar => calcular_longitud_utf8_dolar(valor),
        }
    }
}
