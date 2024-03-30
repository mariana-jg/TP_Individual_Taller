use std::collections::VecDeque;
use std::str::Chars;

use crate::caracter::Caracter;
use crate::errors::Error;
use crate::repeticion::Repeticion;
use crate::step_evaluado::StepEvaluado;
use crate::step_regex::StepRegex;

pub struct Regex {
    pasos: Vec<StepRegex>,
    //anchorings: Anchoring,
}
//[abcd] o [a-d] o [a-dA-Dp-s] => [a,b,c,d] o [a,-,d]
fn conseguir_lista(chars_iter: &mut Chars<'_>) -> Vec<char> {
    let mut auxiliar: Vec<char> = Vec::new();
    let mut contenido: Vec<char> = Vec::new();
    let mut hay_guion = false;

    while let Some(c) = chars_iter.next() {
        if c == ']' {
            break;
        } else {
            auxiliar.push(c);
        }
    }

    for i in 0..auxiliar.len() {
        if auxiliar[i] == '-' {
            hay_guion = true;
            let inicio = auxiliar[i - 1];
            let fin = auxiliar[i + 1];
            for c in inicio..=fin {
                contenido.push(c);
            }
        }        }

        if !hay_guion {
        for i in 0..auxiliar.len() {
                    contenido.push(auxiliar[i]);
            }        
        }
    /*if !hay_guion {
        for i in 0..auxiliar.len() {
            contenido.push(auxiliar[i]);
        }
        return contenido;
    }
    let inicio = auxiliar[0];
    let fin = auxiliar[auxiliar.len() - 1];
    for c in inicio..=fin {
        contenido.push(c);
    }*/

    println!("{:?}", contenido);
    contenido
}

pub fn agregar_pasos(
    steps: &mut Vec<StepRegex>,
    chars_iter: &mut Chars<'_>,
) -> Result<Vec<StepRegex>, Error> {
    while let Some(c) = chars_iter.next() {
        let step = match c {
            '.' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Wildcard,
            }),

            'a'..='z' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Literal(c),
            }),

            'A'..='Z' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Literal(c),
            }),

            '0'..='9' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Literal(c),
            }),

            //'$' => {return None},
            '{' => {

                if let Some(last) = steps.last_mut() {
                    let mut contenido: Vec<char> = Vec::new();
                    let mut rangos: Vec<usize> = Vec::new();
                    while let Some(c) = chars_iter.next() {
                        if c == ',' {
                            contenido.push(c);
                        } else if c == '}' {
                            break;
                        } else {
                            contenido.push(c);
                            match c.to_string().parse::<usize>() {
                                Ok(cant) => rangos.push(cant),
                                Err(_) => return Err(Error::CaracterNoProcesable),
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
                        last.repeticiones = Repeticion::Exacta(rangos[0]);
                    } else {
                        return Err(Error::CaracterNoProcesable);
                    }
                }
                None
            }

            '[' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Lista(conseguir_lista(chars_iter)),
            }),

            '?' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Rango {
                        min: Some(0),
                        max: Some(1),
                    }
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            }

            '*' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Alguna;
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            }

            '+' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Rango {
                        min: Some(1),
                        max: None,
                    };
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            }

            '\\' => match chars_iter.next() {
                Some(literal) => Some(StepRegex {
                    repeticiones: Repeticion::Exacta(1),
                    caracter_interno: Caracter::Literal(literal),
                }),
                None => return Err(Error::CaracterNoProcesable),
            },

            _ => return Err(Error::CaracterNoProcesable),
        };

        if let Some(p) = step {
            steps.push(p);
        }
    }

    Ok(steps.to_vec())
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, Error> {
        let mut steps: Vec<StepRegex> = Vec::new();
        let mut chars_iter = expression.chars();

        /*  if !expression.starts_with('^') {
                    let paso = Some(StepRegex {
            repeticiones: Repeticion::Alguna,
            caracter_interno: Caracter::Wildcard,
            });
            if let Some(p) = paso {
                steps.push(p);
            }
            if let Some(last) = steps.last_mut() {
                last.repeticiones = Repeticion::Alguna;
            } else {
                return Err(Error::CaracterNoProcesable);
            }


        }

        chars_iter.next();   */

        let steps = agregar_pasos(&mut steps, &mut chars_iter)?;

        /*   if expression.ends_with('$') {
            let paso = Some(StepRegex {
                repeticiones: Repeticion::Exacta(1),
                caracter_interno: Caracter::Literal('\0'),
            });
            if let Some(p) = paso {
                steps.push(p);
            }
        }       */
        Ok(Regex { pasos: steps })
    }

    pub fn es_valida(self, linea: &str) -> Result<bool, Error> {
        if !linea.is_ascii() {
            return Err(Error::FormatoDeLineaNoASCII);
        }
        let mut queue: VecDeque<StepRegex> = VecDeque::from(self.pasos);
        let mut stack: Vec<StepEvaluado> = Vec::new();
        let mut index = 0;

        'pasos: while let Some(paso) = queue.pop_front() {
            match paso.repeticiones {
                Repeticion::Exacta(n) => {
                    let mut match_size = 0;
                    for _ in 0..n {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);
                        println!("hay coincidencia: {:?}", paso.caracter_interno);
                        if avance == 0 {
                            match backtrack(paso, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'pasos;
                                }
                                None => return Ok(false),
                            }
                        } else {
                            match_size += avance;
                            index += avance;
                        }
                    }

                    stack.push(StepEvaluado {
                        paso: paso,
                        match_size: match_size,
                        backtrackable: false,
                    });
                }
                Repeticion::Alguna => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            stack.push(StepEvaluado {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackable: true,
                            })
                        } else {
                            sigo_avanzando = false;
                        }
                    }
                }
                Repeticion::Rango { min, max } => {
                    let min = match min {
                        Some(min) => min,
                        None => 0,
                    };

                    let max = match max {
                        Some(max) => max,
                        None => linea.len() - index,
                    };
                    let mut aux: Vec<StepEvaluado> = Vec::new();

                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            aux.push(StepEvaluado {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackable: true,
                            });
                            stack.push(StepEvaluado {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackable: true,
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
    current: StepRegex,
    evaluated: &mut Vec<StepEvaluado>,
    next: &mut VecDeque<StepRegex>,
) -> Option<usize> {
    let mut back_size = 0;
    next.push_front(current);
    while let Some(e) = evaluated.pop() {
        back_size += e.match_size;
        if e.backtrackable {
            println!("backtrack {}", back_size);
            return Some(back_size);
        } else {
            next.push_front(e.paso);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_regex_con_literales() {
        let regex = Regex::new("abc");
        assert_eq!(regex.unwrap().es_valida("abcdefg").unwrap(), true);
    }

    #[test]
    fn test02_regex_con_comodin() {
        let regex = Regex::new("ab.c");
        assert_eq!(regex.unwrap().es_valida("abacdefg").unwrap(), true);
    }

    #[test]
    fn test03_regex_con_asterisk() {
        let regex = Regex::new("ab*c");
        assert_eq!(regex.unwrap().es_valida("abbbbbbc").unwrap(), true);
    }

    #[test]
    fn test04_regex_con_metacaracter_con_backlash() {
        let regex = Regex::new("a\\*");
        assert_eq!(regex.unwrap().es_valida("a*cds").unwrap(), true);
    }

    #[test]
    fn test05_regex_con_plus() {
        let regex = Regex::new("hola+");
        assert_eq!(regex.unwrap().es_valida("holaa").unwrap(), true);
    }

    #[test]
    fn test06_regex_con_plus() {
        let regex = Regex::new("hola+");
        assert_eq!(regex.unwrap().es_valida("hol").unwrap(), false);
    }

    #[test]
    fn test07_regex_con_question() {
        let regex = Regex::new("hola?f");
        assert_eq!(regex.unwrap().es_valida("holaf").unwrap(), true);
    }

    #[test]
    fn test08_regex_con_question2() {
        let regex = Regex::new("hola?s");
        assert_eq!(regex.unwrap().es_valida("hols").unwrap(), true);
    }

    #[test]
    fn test09_regex_con_question3() {
        let regex = Regex::new("hola?");
        assert_eq!(regex.unwrap().es_valida("holaaaaa").unwrap(), false);
    }

    #[test]
    fn test10_regex_con_bracket_exacto() {
        let regex = Regex::new("a{2}");
        assert_eq!(regex.unwrap().es_valida("a").unwrap(), false);
    }

    #[test]
    fn test11_regex_con_bracket_exacto_() {
        let regex = Regex::new("ba{2}");
        assert_eq!(regex.unwrap().es_valida("baa").unwrap(), true);
    }

    #[test]
    fn test12_regex_con_bracket_exacto_() {
        let regex = Regex::new("ba{2}c");
        assert_eq!(regex.unwrap().es_valida("bac").unwrap(), false);
    }

    #[test]
    fn test13_regex_con_bracket_con_minimo_() {
        let regex = Regex::new("ba{2,}c");
        assert_eq!(regex.unwrap().es_valida("baaaac").unwrap(), true);
    }

    #[test]
    fn test14_regex_con_bracket_con_minimo_() {
        let regex = Regex::new("ba{2,}c");
        assert_eq!(regex.unwrap().es_valida("bac").unwrap(), false);
    }

    #[test]
    fn test15_regex_con_bracket_con_rango_() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaac").unwrap(), true);
    }

    #[test]
    fn test16_regex_con_bracket_con_rango_() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaac").unwrap(), false);
    }

    #[test]
    fn test17_regex_con_bracket_con_rango_() {
        let regex = Regex::new("ba{5,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaaaaaac").unwrap(), false);
    }

    #[test]
    fn test18_regex_con_bracket_con_maximo1_() {
        let regex = Regex::new("ba{,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaaac").unwrap(), true);
    }

    #[test]
    fn test19_regex_con_bracket_con_maximo2_() {
        let regex = Regex::new("ba{,8}c");
        assert_eq!(regex.unwrap().es_valida("baaaaaaaaaaaaaaaac").unwrap(), false);
    }
    #[test]
    fn test20_regex_combinado() {
        let regex = Regex::new("ba{5,8}.c");
        assert_eq!(regex.unwrap().es_valida("baaaaaaafc").unwrap(), true);
    }

    #[test]
    fn test21_regex_bracket_literal_01() {
        let regex = Regex::new("ho[lmn]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test22_regex_bracket_literal_02() {
        let regex = Regex::new("ho[lmn]a");
        assert_eq!(regex.unwrap().es_valida("hoka").unwrap(), false);
    }

    #[test]
    fn test23_regex_bracket_rango_01() {
        let regex = Regex::new("ho[i-m]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test24_regex_bracket_rango_02() {
        let regex = Regex::new("ho[i-m]a");
        assert_eq!(regex.unwrap().es_valida("hosa").unwrap(), false);
    }

    #[test]
    fn test25_regex_combinado() {
        let regex = Regex::new("ho[k-o]a.p{2,4}");
        assert_eq!(regex.unwrap().es_valida("hola3ppp").unwrap(), true);
    }

    #[test]
    fn test26_regex_combinado() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoAa").unwrap(), true);
    }

    #[test]
    fn test27_regex_combinado() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoXa").unwrap(), false);
    }
  
}
