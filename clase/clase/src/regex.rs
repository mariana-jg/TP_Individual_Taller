use std::collections::{HashSet, VecDeque};
use std::str::Chars;

use crate::caracter::Caracter;
use crate::clase_char::ClaseChar;
use crate::errors::Error;
use crate::paso_evaluado::PasoEvaluado;
use crate::paso_regex::PasoRegex;
use crate::repeticion::Repeticion;

const CORCHETE_ABIERTO: char = '[';
const CORCHETE_CERRADO: char = ']';
const LLAVE_ABIERTA: char = '{';
const LLAVE_CERRADA: char = '}';
const ASTERISCO: char = '*';
const INTERROGACION: char = '?';
const MAS: char = '+';
const PUNTO: char = '.';
const BARRA: char = '\\';
const DOLAR: char = '$';
const CARET: char = '^';
const INDICADOR_CLASE: char = ':';
const SEPARADOR_RANGO: char = '-';
const FUNCION_OR: char = '|';

pub struct Regex {
    pasos: Vec<PasoRegex>,
}

fn obtener_auxiliar(chars_iter: &mut Chars<'_>) -> (Vec<char>, bool, bool) {
    let mut cantidad_corchetes = 1;
    let mut hay_clase = false;
    let mut es_negado = false;
    let mut auxiliar: Vec<char> = Vec::new();

    for c in chars_iter.by_ref() {
        match c {
            CORCHETE_CERRADO if cantidad_corchetes == 2 || c == CORCHETE_CERRADO && !hay_clase => {
                break
            }
            CORCHETE_CERRADO => cantidad_corchetes += 1,
            CARET => es_negado = true,
            INDICADOR_CLASE => continue,
            CORCHETE_ABIERTO => hay_clase = true,
            _ => auxiliar.push(c),
        }
    }

    (auxiliar, hay_clase, es_negado)
}

fn determinar_contenido_a_evaluar(auxiliar: Vec<char>) -> Result<HashSet<char>, Error> {
    let mut contenido: HashSet<char> = HashSet::new();

    for i in 0..auxiliar.len() {
        if auxiliar[i] == SEPARADOR_RANGO {
            if let (Some(inicio), Some(fin)) = (auxiliar.get(i - 1), auxiliar.get(i + 1)) {
                contenido.extend(*inicio..=*fin);
            }
        } else {
            contenido.insert(auxiliar[i]);
        }
    }
    Ok(contenido)
}

fn conseguir_lista(chars_iter: &mut Chars<'_>) -> Result<(ClaseChar, bool), Error> {
    let (auxiliar, hay_clase, es_negado) = obtener_auxiliar(chars_iter);

    if hay_clase {
        let class: String = auxiliar.iter().collect();

        match class.to_string().as_str() {
            "alpha" => return Ok((ClaseChar::Alpha, es_negado)),
            "alnum" => return Ok((ClaseChar::Alnum, es_negado)),
            "digit" => return Ok((ClaseChar::Digit, es_negado)),
            "lower" => return Ok((ClaseChar::Lower, es_negado)),
            "upper" => return Ok((ClaseChar::Upper, es_negado)),
            "space" => return Ok((ClaseChar::Space, es_negado)),
            "punct" => return Ok((ClaseChar::Punct, es_negado)),
            _ => {}
        }
    }

    let contenido = determinar_contenido_a_evaluar(auxiliar);

    match contenido {
        Ok(content) => Ok((ClaseChar::Simple(content), es_negado)),
        Err(error) => Err(error),
    }
}

fn no_hay_anterior(anterior: &mut PasoRegex) -> bool {
    anterior.caracter_interno == Caracter::Comodin
        && anterior.repeticiones == Repeticion::Alguna(false)
}

pub fn agregar_pasos(
    steps: &mut Vec<PasoRegex>,
    chars_iter: &mut Chars<'_>,
) -> Result<Vec<PasoRegex>, Error> {
    while let Some(c) = chars_iter.next() {
        let step = match c {
            PUNTO => Some(PasoRegex {
                repeticiones: Repeticion::Exacta(1, false),
                caracter_interno: Caracter::Comodin,
            }),

            'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' => Some(PasoRegex {
                repeticiones: Repeticion::Exacta(1, false),
                caracter_interno: Caracter::Literal(c),
            }),

            LLAVE_ABIERTA => {
                if let Some(last) = steps.last_mut() {
                    if no_hay_anterior(last) {
                        return Err(Error::CaracterNoProcesable);
                    } else if let Some(last) = steps.last_mut() {
                        let mut contenido: Vec<char> = Vec::new();
                        let mut rangos: Vec<usize> = Vec::new();
                        for c in chars_iter.by_ref() {
                            if c == ',' {
                                contenido.push(c);
                            } else if c == LLAVE_CERRADA {
                                break;
                            } else {
                                contenido.push(c);
                                match c.to_string().parse::<usize>() {
                                    Ok(cant) => rangos.push(cant),
                                    Err(_) => return Err(Error::ErrorEnLlaves),
                                }
                            }
                        }

                        if contenido.len() >= 2 {
                            if contenido[0] == ',' {
                                last.repeticiones = Repeticion::Rango {
                                    min: None,
                                    max: Some(rangos[0]),
                                };
                            } else if contenido[contenido.len() - 1] == ',' {
                                last.repeticiones = Repeticion::Rango {
                                    min: Some(rangos[0]),
                                    max: None,
                                };
                            } else {
                                last.repeticiones = Repeticion::Rango {
                                    min: Some(rangos[0]),
                                    max: Some(rangos[1]),
                                };
                            }
                        } else if contenido.len() == 1 && contenido[0].is_ascii_digit() {
                            last.repeticiones = Repeticion::Exacta(rangos[0], false);
                        } else {
                            return Err(Error::ErrorEnLlaves);
                        }
                    }
                }

                None
            }

            CORCHETE_ABIERTO => match conseguir_lista(chars_iter) {
                Ok(contenido) => Some(PasoRegex {
                    repeticiones: Repeticion::Exacta(1, contenido.1),
                    caracter_interno: Caracter::Serie(contenido.0),
                }),
                Err(error) => return Err(error),
            },

            INTERROGACION => {
                if let Some(last) = steps.last_mut() {
                    if no_hay_anterior(last) {
                        return Err(Error::ErrorEnRepeticion);
                    } else {
                        last.repeticiones = Repeticion::Rango {
                            min: Some(0),
                            max: Some(1),
                        };
                    }
                }
                None
            }

            ASTERISCO => {
                if let Some(last) = steps.last_mut() {
                    if no_hay_anterior(last) {
                        return Err(Error::ErrorEnRepeticion);
                    } else {
                        match conseguir_lista(chars_iter) {
                            Ok((_, negado)) => last.repeticiones = Repeticion::Alguna(negado),
                            Err(error) => return Err(error),
                        };
                    }
                }
                None
            }

            MAS => {
                if let Some(last) = steps.last_mut() {
                    if no_hay_anterior(last) {
                        return Err(Error::ErrorEnRepeticion);
                    } else {
                        last.repeticiones = Repeticion::Rango {
                            min: Some(1),
                            max: None,
                        };
                    }
                }
                None
            }

            BARRA => match chars_iter.next() {
                Some(literal) => Some(PasoRegex {
                    repeticiones: Repeticion::Exacta(1, false),
                    caracter_interno: Caracter::Literal(literal),
                }),
                None => return Err(Error::CaracterNoProcesable),
            },

            DOLAR => Some(PasoRegex {
                repeticiones: Repeticion::Exacta(1, false),
                caracter_interno: Caracter::Dolar,
            }),

            CARET => None,

            FUNCION_OR => None,

            _ => return Err(Error::CaracterNoProcesable),
        };

        if let Some(p) = step {
            steps.push(p);
        }
    }

    Ok(steps.to_vec())
}

fn definir_uso_de_caret(expression: &str, steps: &mut Vec<PasoRegex>) {
    if !expression.starts_with(CARET) {
        let paso = Some(PasoRegex {
            repeticiones: Repeticion::Alguna(false),
            caracter_interno: Caracter::Comodin,
        });
        if let Some(p) = paso {
            steps.push(p);
        }
    }
}

fn expresion_escrita_correctamente(expresion: &str) -> Result<(), Error> {
    let mut iter = expresion.chars();
    let mut cont_llaves = 0;
    let mut cont_corchetes = 0;
    while let Some(c) = iter.next() {
        match c {
            LLAVE_ABIERTA => cont_llaves += 1,
            LLAVE_CERRADA => cont_llaves -= 1,
            CORCHETE_ABIERTO => cont_corchetes += 1,
            CORCHETE_CERRADO => cont_corchetes -= 1,
            FUNCION_OR => {
                if cont_llaves != 0 || cont_corchetes != 0 {
                    return Err(Error::ErrorEnFuncionOR);
                }
            }
            _ => {}
        }
    }
    if cont_llaves != 0 {
        return Err(Error::ErrorEnLlaves);
    }
    if cont_corchetes != 0 {
        return Err(Error::ErrorEnCorchetes);
    }
    Ok(())
}
impl Regex {
    pub fn es_valida_general(expresion_completa: &str, linea: &str) -> Result<bool, Error> {
        let mut valida = false;

        match expresion_escrita_correctamente(expresion_completa) {
            Ok(_) => {
                let expresiones_a_evaluar: Vec<&str> = expresion_completa.split('|').collect();

                for exp in expresiones_a_evaluar {
                    let regex = match Regex::new(exp) {
                        Ok(regex) => regex,
                        Err(err) => return Err(err),
                    };
                    if regex.es_valida(linea)? {
                        valida = true;
                        break;
                    }
                }
                Ok(valida)
            }
            Err(error) => return Err(error),
        }
    }

    pub fn new(expression: &str) -> Result<Self, Error> {
        let mut steps: Vec<PasoRegex> = Vec::new();
        let mut chars_iter = expression.chars();

        definir_uso_de_caret(expression, &mut steps);

        let steps: Vec<PasoRegex> = agregar_pasos(&mut steps, &mut chars_iter)?;

        Ok(Regex { pasos: steps })
    }

    pub fn es_valida(self, linea: &str) -> Result<bool, Error> {
        if !linea.is_ascii() {
            return Err(Error::FormatoDeLineaNoASCII);
        }
        let mut cola: VecDeque<PasoRegex> = VecDeque::from(self.pasos);
        let mut pila: Vec<PasoEvaluado> = Vec::new();
        let mut index = 0;

        'pasos: while let Some(paso) = cola.pop_front() {
            match paso.repeticiones {
                Repeticion::Exacta(n, negacion) => {
                    let mut tam_coincidencia = 0;
                    for _ in 0..n {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);
                        if avance == 0 {
                            match backtrack(paso, &mut pila, &mut cola) {
                                Some(size) => {
                                    index -= size;
                                    continue 'pasos;
                                }
                                None => {
                                    if negacion {
                                        return Ok(true);
                                    } else {
                                        return Ok(false);
                                    }
                                }
                            }
                        } else {
                            tam_coincidencia += avance;
                            index += avance;
                        }
                    }

                    pila.push(PasoEvaluado {
                        paso,
                        tam_matcheo: tam_coincidencia,
                        backtrackeable: false,
                    });
                    if negacion {
                        return Ok(false);
                    }
                }
                Repeticion::Alguna(negacion) => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            pila.push(PasoEvaluado {
                                paso: paso.clone(),
                                tam_matcheo: avance,
                                backtrackeable: true,
                            })
                        } else {
                            sigo_avanzando = false;
                        }
                        if negacion {
                            return Ok(false);
                        }
                    }
                }
                Repeticion::Rango { min, max } => {
                    let min = min.unwrap_or(0);

                    let max = match max {
                        Some(max) => max,
                        None => linea.len() - index,
                    };
                    let mut aux: Vec<PasoEvaluado> = Vec::new();

                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            aux.push(PasoEvaluado {
                                paso: paso.clone(),
                                tam_matcheo: avance,
                                backtrackeable: true,
                            });
                            pila.push(PasoEvaluado {
                                paso: paso.clone(),
                                tam_matcheo: avance,
                                backtrackeable: true,
                            })
                        } else {
                            sigo_avanzando = false;
                        }
                    }

                    if aux.len() < min || aux.len() > max {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }
}

fn backtrack(
    actual: PasoRegex,
    evaluados: &mut Vec<PasoEvaluado>,
    siguiente: &mut VecDeque<PasoRegex>,
) -> Option<usize> {
    let mut back_size = 0;

    siguiente.push_front(actual);

    while let Some(paso_ev) = evaluados.pop() {
        back_size += paso_ev.tam_matcheo;
        if paso_ev.backtrackeable {
            return Some(back_size);
        } else {
            siguiente.push_front(paso_ev.paso);
        }
    }
    None
}

///test unitarios
#[cfg(test)]
mod tests {
    use super::*;
    ///Del test01 al test71 se pruebas funcionalidades basicas. Luego se prueban combinaciones.
    ///Del test72 al test79 se prueban combinaciones
    /// Del test79 en adelante probamos expresiones cuya regex puede dividirse en 2 partes.
    #[test]
    fn test01_literales() {
        let regex = Regex::new("abcd");
        assert_eq!(regex.unwrap().es_valida("abcdefg").unwrap(), true);
    }

    #[test]
    fn test02_literales() {
        let regex = Regex::new("^abcd");
        assert_eq!(regex.unwrap().es_valida("abcdefg").unwrap(), true);
    }

    #[test]
    fn test03_literales() {
        let regex = Regex::new("^abcd");
        assert_eq!(regex.unwrap().es_valida("ab abcdefg").unwrap(), false);
    }

    #[test]
    fn test04_literales() {
        let regex = Regex::new("abcd");
        assert_eq!(regex.unwrap().es_valida("efgabcd").unwrap(), true);
    }

    #[test]
    fn test05_literales() {
        let regex = Regex::new("abcd");
        assert_eq!(regex.unwrap().es_valida("abcefg").unwrap(), false);
    }

    #[test]
    fn test06_punto() {
        let regex = Regex::new("ab.cd");
        assert_eq!(regex.unwrap().es_valida("ab0cd").unwrap(), true);
    }

    #[test]
    fn test07_punto() {
        let regex = Regex::new("ab.cd");
        assert_eq!(regex.unwrap().es_valida("abcd").unwrap(), false);
    }

    #[test]
    fn test08_regex_con_asterisk() {
        let regex = Regex::new("ab*c");
        assert_eq!(regex.unwrap().es_valida("abbbbbbcd").unwrap(), true);
    }

    #[test]
    fn test08_punto_asterisco() {
        let regex = Regex::new("ab.*cd");
        assert_eq!(regex.unwrap().es_valida("abcd").unwrap(), true);
    }

    #[test]
    fn test09_punto_asterisco() {
        let regex = Regex::new("ab.*cd");
        assert_eq!(regex.unwrap().es_valida("abaaaaaacd").unwrap(), true);
    }

    #[test]
    fn test10_corchete() {
        let regex = Regex::new("a[bc]d");
        assert_eq!(regex.unwrap().es_valida("abd").unwrap(), true);
    }

    #[test]
    fn test11_corchete() {
        let regex = Regex::new("a[bc]d");
        assert_eq!(regex.unwrap().es_valida("acd").unwrap(), true);
    }

    #[test]
    fn test12_corchete() {
        let regex = Regex::new("a[bc]d");
        assert_eq!(regex.unwrap().es_valida("afd").unwrap(), false);
    }

    #[test]
    fn test13_barra() {
        let regex = Regex::new("a\\*");
        assert_eq!(regex.unwrap().es_valida("a*cds").unwrap(), true);
    }

    #[test]
    fn test14_mas() {
        let regex = Regex::new("hola+");
        assert_eq!(regex.unwrap().es_valida("holaa").unwrap(), true);
    }

    #[test]
    fn test15_mas() {
        let regex = Regex::new("hola+");
        assert_eq!(regex.unwrap().es_valida("hol").unwrap(), false);
    }

    #[test]
    fn test16_interrogacion() {
        let regex = Regex::new("holi?s");
        assert_eq!(regex.unwrap().es_valida("holis").unwrap(), true);
    }

    #[test]
    fn test17_interrogacion() {
        let regex = Regex::new("holi?s");
        assert_eq!(regex.unwrap().es_valida("hols").unwrap(), true);
    }

    #[test]
    fn test18_interrogacion() {
        let regex = Regex::new("hola?");
        assert_eq!(regex.unwrap().es_valida("holaaaaa").unwrap(), false);
    }

    #[test]
    fn test19_llave_exacto() {
        let regex = Regex::new("a{2}");
        assert_eq!(regex.unwrap().es_valida("a").unwrap(), false);
    }

    #[test]
    fn test20_llave_exacto() {
        let regex = Regex::new("ba{2}");
        assert_eq!(regex.unwrap().es_valida("baa").unwrap(), true);
    }

    #[test]
    fn test21_llave_exacto() {
        let regex = Regex::new("ba{2}c");
        assert_eq!(regex.unwrap().es_valida("bac").unwrap(), false);
    }

    #[test]
    fn test22_llave_minimo() {
        let regex = Regex::new("ba{2,}c");
        assert_eq!(regex.unwrap().es_valida("baaaac").unwrap(), true);
    }

    #[test]
    fn test23_llave_minimo() {
        let regex = Regex::new("ba{2,}c");
        assert_eq!(regex.unwrap().es_valida("bac").unwrap(), false);
    }

    #[test]
    fn test24_llave_rango() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaac").unwrap(), true);
    }

    #[test]
    fn test25_llave_rango() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaac").unwrap(), false);
    }

    #[test]
    fn test26_llave_rango() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaaaaaac").unwrap(), false);
    }

    #[test]
    fn test27_llave_maximo() {
        let regex = Regex::new("ba{,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaaac").unwrap(), true);
    }

    #[test]
    fn test28_llave_maximo() {
        let regex = Regex::new("ba{,8}c");
        assert_eq!(
            regex.unwrap().es_valida("baaaaaaaaaaaaaaaac").unwrap(),
            false
        );
    }

    #[test]
    fn test29_corchete_literal() {
        let regex = Regex::new("ho[lmn]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test30_corchete_literal() {
        let regex = Regex::new("ho[lmn]a");
        assert_eq!(regex.unwrap().es_valida("hoka").unwrap(), false);
    }

    #[test]
    fn test31_corchete_rango() {
        let regex = Regex::new("ho[i-m]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test32_corchete_rango() {
        let regex = Regex::new("ho[i-m]a");
        assert_eq!(regex.unwrap().es_valida("hosa").unwrap(), false);
    }

    #[test]
    fn test33_corchete_rango() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoAa").unwrap(), true);
    }

    #[test]
    fn test34_corchete_rango() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoXa").unwrap(), false);
    }

    #[test]
    fn test35_corchete_rango_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoXa").unwrap(), true);
    }

    #[test]
    fn test36_corchete_rango_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoxa").unwrap(), false);
    }

    #[test]
    fn test37_corchete_interrogacion() {
        let regex = Regex::new("ho[a-dA-Cx-z]?a");
        assert_eq!(regex.unwrap().es_valida("hoddda").unwrap(), false);
    }

    #[test]
    fn test38_corchete_interrogacion() {
        let regex = Regex::new("ho[a-dA-Cx-z]?a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test39_corchete_interrogacion() {
        let regex = Regex::new("ho[d-g]?a");
        assert_eq!(regex.unwrap().es_valida("hoea").unwrap(), true);
    }

    #[test]
    fn test40_corchete_mas() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(regex.unwrap().es_valida("hoE").unwrap(), false);
    }

    #[test]
    fn test41_corchete_mas() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(regex.unwrap().es_valida("hoAAAAAa").unwrap(), true);
    }

    #[test]
    fn test42_corchete_mas() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(regex.unwrap().es_valida("hoxxxAAAAa").unwrap(), true);
    }

    #[test]
    fn test43_corchete_llave() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoaE").unwrap(), false);
    }

    #[test]
    fn test44_corchete_llave() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoxxaE").unwrap(), true);
    }

    #[test]
    fn test45_corchete_llave() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoCCCCCa").unwrap(), false);
    }

    #[test]
    fn test46_corchete_asterisco() {
        let regex = Regex::new("ho[a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test47_corchete_asterisco() {
        let regex = Regex::new("ho[a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoAAAa").unwrap(), true);
    }

    #[test]
    fn test48_corchete_asterisco_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoKa").unwrap(), true);
    }

    #[test]
    fn test49_corchete_asterisco_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test50_clase_alpha() {
        let regex = Regex::new("ho[[:alpha:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test51_clase_alpha_negada() {
        let regex = Regex::new("ho[^[:alpha:]]a");
        assert_eq!(regex.unwrap().es_valida("ho8a").unwrap(), true);
    }

    #[test]
    fn test52_clase_alnum() {
        let regex = Regex::new("ho[[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("hoKa").unwrap(), true);
    }

    #[test]
    fn test53_clase_alnum() {
        let regex = Regex::new("ho[[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("ho4a").unwrap(), true);
    }

    #[test]
    fn test54_clase_alnum_negada() {
        let regex = Regex::new("ho[^[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("ho&a").unwrap(), true);
    }

    #[test]
    fn test55_clase_digit() {
        let regex = Regex::new("ho[[:digit:]]a");
        assert_eq!(regex.unwrap().es_valida("ho2a").unwrap(), true);
    }

    #[test]
    fn test56_clase_digit_mas() {
        let regex = Regex::new("ho[[:digit:]]+");
        assert_eq!(regex.unwrap().es_valida("ho9999999").unwrap(), true);
    }

    #[test]
    fn test57_clase_digit_negada() {
        let regex = Regex::new("ho[^[:digit:]]a");
        assert_eq!(regex.unwrap().es_valida("hoea").unwrap(), true);
    }

    #[test]
    fn test58_clase_lower() {
        let regex = Regex::new("ho[[:lower:]]a");
        assert_eq!(regex.unwrap().es_valida("hoRa").unwrap(), false);
    }

    #[test]
    fn test59_clase_lower() {
        let regex = Regex::new("ho[[:lower:]]a");
        assert_eq!(regex.unwrap().es_valida("hora").unwrap(), true);
    }

    #[test]
    fn test60_clase_upper() {
        let regex = Regex::new("ho[[:upper:]]a");
        assert_eq!(regex.unwrap().es_valida("hoRa").unwrap(), true);
    }

    #[test]
    fn test61_clase_upper() {
        let regex = Regex::new("ho[[:upper:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), false);
    }

    #[test]
    fn test62_clase_space() {
        let regex = Regex::new("ho[[:space:]]a");
        assert_eq!(regex.unwrap().es_valida("ho a").unwrap(), true);
    }

    #[test]
    fn test63_clase_space() {
        let regex = Regex::new("ho[[:space:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), false);
    }

    #[test]
    fn test64_clase_punct() {
        let regex = Regex::new("ho[[:punct:]]a");
        assert_eq!(regex.unwrap().es_valida("ho;a").unwrap(), true);
    }

    #[test]
    fn test65_clase_punct() {
        let regex = Regex::new("ho[[:punct:]]a");
        assert_eq!(regex.unwrap().es_valida("ho9a").unwrap(), false);
    }

    #[test]
    fn test66_dolar() {
        let regex = Regex::new("hola$");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test67_dollar() {
        let regex = Regex::new("hola$");
        assert_eq!(regex.unwrap().es_valida("el dijo: hola").unwrap(), true);
    }

    #[test]
    fn test68_dollar() {
        let regex = Regex::new("hola$");
        assert_eq!(regex.unwrap().es_valida("el dijo: hol").unwrap(), false);
    }

    #[test]
    fn test69_dolar_caret() {
        let regex = Regex::new("^hola$");
        assert_eq!(regex.unwrap().es_valida("el dijo: hola").unwrap(), false);
    }

    #[test]
    fn test70_dolar_caret() {
        let regex = Regex::new("^hola$");
        assert_eq!(regex.unwrap().es_valida("hola me dijo el").unwrap(), false);
    }

    #[test]
    fn test71_dolar_caret() {
        let regex = Regex::new("^hola$");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test72_combinado() {
        let regex = Regex::new("ba{5,8}.c");
        assert_eq!(regex.unwrap().es_valida("baaaaaaafc").unwrap(), true);
    }

    #[test]
    fn test73_combinado() {
        let regex = Regex::new("ho[k-o]a.p{2,4}");
        assert_eq!(regex.unwrap().es_valida("hola3ppp").unwrap(), true);
    }

    #[test]
    fn test74_combinado() {
        let regex = Regex::new("^ho[[:punct:]]{2}a+");
        assert_eq!(regex.unwrap().es_valida("ho..aaaaaa").unwrap(), true);
    }

    #[test]
    fn test75_combinado() {
        let regex = Regex::new("ho[[:punct:]]{2}a+");
        assert_eq!(regex.unwrap().es_valida("aaaaa ho..aaaaaa").unwrap(), true);
    }

    #[test]
    fn test76_combinado() {
        let regex = Regex::new("[a-kA-G]ho[[:punct:]]*a\\.?");
        assert_eq!(regex.unwrap().es_valida("Dho;.a.").unwrap(), true);
    }

    #[test]
    fn test77_combinado() {
        let regex = Regex::new("^hola [[:alpha:]]+");
        assert_eq!(regex.unwrap().es_valida("hola como estas").unwrap(), true);
    }

    #[test]
    fn test78_combinado() {
        let regex = Regex::new("^hola [[:alpha:]]+");
        assert_eq!(
            regex
                .unwrap()
                .es_valida("el me dijo: hola como estas")
                .unwrap(),
            false
        );
    }

    #[test]
    fn test79_combinado() {
        let regex = Regex::new("[[:upper:]]ascal[[:upper:]]ase");
        assert_eq!(regex.unwrap().es_valida("PascalCase").unwrap(), true);
    }

    #[test]
    fn test80_combinado() {
        let regex = Regex::new("[[:upper:]]ascal[[:upper:]]ase");
        assert_eq!(regex.unwrap().es_valida("Pascalcase").unwrap(), false);
    }

    #[test]
    fn test81_combinacion_general() {
        assert_eq!(
            Regex::es_valida_general("[abc]d[[:alpha:]]|k", "hola").unwrap(),
            false
        );
    }

    #[test]
    fn test82_combinacion_general() {
        assert_eq!(
            Regex::es_valida_general("[abc]d[[:alpha:]]|k", "adAk").unwrap(),
            true
        );
    }

    #[test]
    fn test83_funcion_or() {
        assert_eq!(Regex::es_valida_general("abc|de+f", "abc").unwrap(), true);
    }

    #[test]
    fn test84_funcion_or() {
        assert_eq!(
            Regex::es_valida_general("abc|de+f", "deeeeeeeeeeeeeef").unwrap(),
            true
        );
    }

    #[test]
    fn test85_funcion_or() {
        assert_eq!(
            Regex::es_valida_general("abc|de+f", "abcdeeeeeeeeeeeeeef").unwrap(),
            true
        );
    }

    #[test]
    fn test86_funcion_or() {
        assert_eq!(
            Regex::es_valida_general("abc|de+f", "abdeeeeeeeeeeeeee").unwrap(),
            false
        );
    }
}
