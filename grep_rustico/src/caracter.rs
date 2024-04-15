use crate::clase_char::ClaseChar;

#[derive(Clone, Debug, PartialEq)]
///Representa un caracter que puede ser un literal, un comodín, una serie o un dolar.
/// - El literal es un caracter que se espera que sea exactamente igual al que se está comparando.
/// - El comodín es un caracter que puede ser cualquier caracter.
/// - La serie es un caracter que puede ser cualquier caracter de una clase de caracteres.
/// - El dolar es un caracter que se espera que sea el final de la cadena.
pub enum Caracter {
    Literal(char),
    Comodin,
    Serie(ClaseChar),
    Dolar,
}
///Calcula la longitud en bytes de un caracter de una cadena de texto, si pertenece a una clase de caracter.
fn calcular_longitud_utf8_clase<F>(valor: &str, negado: bool, funcion: F) -> usize
where
    F: Fn(char) -> bool,
{
    if let Some(c) = valor.chars().next() {
        if negado {
            if funcion(c) {
                0
            } else {
                c.len_utf8()
            }
        } else {
            if funcion(c) {
                c.len_utf8()
            } else {
                0
            }
        }
    } else {
        0
    }
}

///Calcula la longitud en bytes de un caracter de una cadena de texto, si es un literal.
fn calcular_longitud_utf8_literal(valor: &str, l: &char) -> usize {
    if valor.starts_with(*l) {
        l.len_utf8()
    } else {
        0
    }
}

///Calcula la longitud en bytes de un caracter de una cadena de texto, si es un comodín.
fn calcular_longitud_utf8_comodin(valor: &str) -> usize {
    if let Some(c) = valor.chars().next() {
        c.len_utf8()
    } else {
        0
    }
}

///Calcula la longitud en bytes de un caracter de una cadena de texto, si es un dolar.
fn calcular_longitud_utf8_dolar(valor: &str) -> usize {
    if valor.chars().next().is_some() {
        0
    } else {
        1
    }
}

impl Caracter {
    ///Según el tipo de caracter con el que estemos trabajando, se calcula su longitud en bytes.
    pub fn coincide(&self, valor: &str) -> usize {
      // println!("valor: {} coincide con {:?}", valor, self);
        match self {
            Caracter::Literal(l) => calcular_longitud_utf8_literal(valor, l),
            Caracter::Comodin => calcular_longitud_utf8_comodin(valor),
            Caracter::Serie(clase) => match clase {
                ClaseChar::Alpha(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, |c: char| {
                        char::is_ascii_alphabetic(&c)
                    })
                }
                ClaseChar::Alnum(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, char::is_alphanumeric)
                }
                ClaseChar::Digit(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, |c| c.is_ascii_digit())
                }
                ClaseChar::Lower(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, char::is_lowercase)
                }
                ClaseChar::Upper(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, char::is_uppercase)
                }
                ClaseChar::Space(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, char::is_whitespace)
                }
                ClaseChar::Punct(negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, |c: char| {
                        char::is_ascii_punctuation(&c)
                    })
                }
                ClaseChar::Simple(list, negado) => {
                    calcular_longitud_utf8_clase(valor, *negado, |c| list.contains(&c))
                }
            },
            Caracter::Dolar => calcular_longitud_utf8_dolar(valor),
        }
    }
}
