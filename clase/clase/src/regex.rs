use std::collections::VecDeque;
use std::str::Chars;

use crate::caracter::Caracter;
use crate::clase_char::ClaseChar;
use crate::errors::Error;
use crate::repeticion::Repeticion;
use crate::step_evaluado::StepEvaluado;
use crate::step_regex::StepRegex;

pub struct Regex {
    pasos: Vec<StepRegex>,
    //anchorings: Anchoring,
}

fn conseguir_lista(chars_iter: &mut Chars<'_>) -> (ClaseChar, bool) {
    
    let mut auxiliar: Vec<char> = Vec::new();
    let mut contenido: Vec<char> = Vec::new();
    let mut hay_guion = false;
    let mut hay_clase = false;
    let mut es_negado = false;
    let mut cantidad_llaves = 0;

    while let Some(c) = chars_iter.next() {
        if c == ']' && cantidad_llaves == 1 || c == ']' && !hay_clase {
            break;
        } else if c == ']' {
            cantidad_llaves += 1;
        } else if c == '^' {
            es_negado = true;
        } else if c == ':' {
            continue;
        } else if c == '[' {
            hay_clase = true
        } else {
            auxiliar.push(c);
        }
    }

    if hay_clase {

        let class: String = auxiliar.iter().collect();

        match class.to_string().as_str() {
            "alpha" => return (ClaseChar::Alpha, es_negado),
            "alnum" => return (ClaseChar::Alnum, es_negado),
            "digit" => return (ClaseChar::Digit, es_negado),
            "lower" => return (ClaseChar::Lower, es_negado),
            "upper" => return (ClaseChar::Upper, es_negado),
            "space" => return (ClaseChar::Space, es_negado),
            "punct" => return (ClaseChar::Punct, es_negado),
            _ => {}
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
        }
    }

    if !hay_guion {
        for i in 0..auxiliar.len() {
            contenido.push(auxiliar[i]);
        }
    }

    (ClaseChar::Simple(contenido), es_negado)

}

pub fn agregar_pasos(
    steps: &mut Vec<StepRegex>,
    chars_iter: &mut Chars<'_>,
) -> Result<Vec<StepRegex>, Error> {
    while let Some(c) = chars_iter.next() {

        let step = match c {

            '.' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1, false),
                caracter_interno: Caracter::Wildcard,
            }),

            'a'..='z' | 'A'..='Z' | '0'..='9' => Some(StepRegex {
                repeticiones: Repeticion::Exacta(1, false),
                caracter_interno: Caracter::Literal(c),
            }),

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
                        last.repeticiones = Repeticion::Exacta(rangos[0], false);
                    } else {
                        return Err(Error::CaracterNoProcesable);
                    }
                }
                None
            }

            '[' => {
                let contenido = conseguir_lista(chars_iter);
                Some(StepRegex {
                    repeticiones: Repeticion::Exacta(1, contenido.1),
                    caracter_interno: Caracter::Lista(contenido.0),
                })
            }

            '?' => {
                if let Some(last) = steps.last_mut() {
                    println!("ultimo paso: {:?}", last);
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
                    let (_, negado) = conseguir_lista(chars_iter);
                    last.repeticiones = Repeticion::Alguna(negado);
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
                    repeticiones: Repeticion::Exacta(1, false),
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

        if !expression.starts_with('^') {
            let paso = Some(StepRegex {
            repeticiones: Repeticion::Alguna(false),
            caracter_interno: Caracter::Wildcard,
            });
            if let Some(p) = paso {
                steps.push(p);
            }

        } else {
            chars_iter.next();
        }

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

        'pasos: while let Some(mut paso) = queue.pop_front() {
            match paso.repeticiones {
                Repeticion::Exacta(n, negacion) => {
                    println!("en este caso, estoy negando: {}", negacion);
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
                                None => {
                                    if negacion {
                                        return Ok(true);
                                    } else {
                                        return Ok(false);
                                    }
                                }
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
                    if negacion {
                        return Ok(false);
                    }
                }
                Repeticion::Alguna(negacion) => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        println!("paso: {:?}", paso.caracter_interno);
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
                        if negacion {
                            return Ok(false);
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
                        if matches!(paso.caracter_interno, Caracter::Lista(_)) {
                            paso.caracter_interno =
                                Caracter::Literal(linea.as_bytes()[index] as char);
                            println!("nuevo paso: {:?}", paso.caracter_interno)
                        }

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
        let regex = Regex::new("^abc");
        assert_eq!(regex.unwrap().es_valida("abcdefg").unwrap(), true);
    }

    #[test]
    fn test02_regex_con_comodin() {
        let regex = Regex::new("ab.c");
        assert_eq!(regex.unwrap().es_valida("eeeea abacdefg").unwrap(), true);
    }

    #[test]
    fn test03_regex_con_asterisk() {
        let regex = Regex::new("ab*c");
        assert_eq!(regex.unwrap().es_valida("abbbbbbc").unwrap(), true);
    }

    #[test]
    fn test03_regex_con_asterisk02() {
        let regex = Regex::new("^ab*c");
        assert_eq!(regex.unwrap().es_valida("fdf ac").unwrap(), false);
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
        assert_eq!(
            regex.unwrap().es_valida("baaaaaaaaaaaaaaaac").unwrap(),
            false
        );
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
    fn test26_regex_bracket_rango_03() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoAa").unwrap(), true);
    }

    #[test]
    fn test27_regex_bracket_rango_04() {
        let regex = Regex::new("ho[a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoXa").unwrap(), false);
    }

    #[test]
    fn test28_regex_bracket_rango_05_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoXa").unwrap(), true);
    }

    #[test]
    fn test29_regex_bracket_rango_06_negado() {
        let regex = Regex::new("ho[^a-dA-Cx-z]");
        assert_eq!(regex.unwrap().es_valida("hoxa").unwrap(), false);
    }

    #[test]
    fn test30_regex_combinado_bracket_question01() {
        let regex = Regex::new("ho[a-dA-Cx-z]?a");
        assert_eq!(regex.unwrap().es_valida("hoddda").unwrap(), false);
    }

    #[test]
    fn test31_regex_combinado_bracket_question02() {
        let regex = Regex::new("ho[a-dA-Cx-z]?a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test32_regex_combinado_bracket_question03() {
        let regex = Regex::new("ho[a-dA-Cx-z]?a");
        assert_eq!(regex.unwrap().es_valida("hoda").unwrap(), true);
    }

    #[test]
    fn test33_regex_combinado_bracket_plus01() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(regex.unwrap().es_valida("hoE").unwrap(), false);
    }

    #[test]
    fn test34_regex_combinado_bracket_plus02() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(regex.unwrap().es_valida("hoaE").unwrap(), true);
    }

    #[test]
    fn test35_regex_combinado_bracket_plus03() {
        let regex = Regex::new("ho[a-dA-Cx-z]+a");
        assert_eq!(
            regex.unwrap().es_valida("hoaaaaaaaaaaaaaaaaaE").unwrap(),
            true
        );
    }

    #[test]
    fn test36_regex_combinado_bracket_rango01() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoaE").unwrap(), false);
    }

    #[test]
    fn test37_regex_combinado_bracket_rango02() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoaaaE").unwrap(), true);
    }

    #[test]
    fn test38_regex_combinado_bracket_rango03() {
        let regex = Regex::new("ho[a-dA-Cx-z]{2,4}a");
        assert_eq!(regex.unwrap().es_valida("hoaaaaaaE").unwrap(), false);
    }

    #[test]
    fn test39_regex_combinado_bracket_asterisk01() {
        let regex = Regex::new("ho[a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test40_regex_combinado_bracket_asterisk02() {
        let regex = Regex::new("ho[a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoAAAa").unwrap(), true);
    }

    #[test]
    fn test41_regex_combinado_bracket_negado_asterisk01() {
        let regex = Regex::new("ho[^a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoKa").unwrap(), true);
    }

    #[test]
    fn test42_regex_combinado_bracket_negado_asterisk02() {
        let regex = Regex::new("ho[^a-dA-Cx-z]*a");
        assert_eq!(regex.unwrap().es_valida("hoa").unwrap(), true);
    }

    #[test]
    fn test43_regex_clases01() {
        let regex = Regex::new("ho[[:alpha:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), true);
    }

    #[test]
    fn test44_regex_clases02() {
        let regex = Regex::new("ho[^[:alpha:]]a");
        assert_eq!(regex.unwrap().es_valida("ho8a").unwrap(), true);
    }

    #[test]
    fn test45_regex_clases03() {
        let regex = Regex::new("ho[[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("hoKa").unwrap(), true);
    }

    #[test]
    fn test46_regex_clases04() {
        let regex = Regex::new("ho[[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("ho4a").unwrap(), true);
    }

    #[test]
    fn test47_regex_clases05() {
        let regex = Regex::new("ho[^[:alnum:]]a");
        assert_eq!(regex.unwrap().es_valida("ho&a").unwrap(), true);
    }

    #[test]
    fn test48_regex_clases06() {
        let regex = Regex::new("ho[[:digit:]]a");
        assert_eq!(regex.unwrap().es_valida("ho2a").unwrap(), true);
    }

    #[test]
    fn test49_regex_clases07() {
        let regex = Regex::new("ho[[:digit:]]a");
        assert_eq!(regex.unwrap().es_valida("hoRa").unwrap(), false);
    }

    #[test]
    fn test50_regex_clases08() {
        let regex = Regex::new("ho[^[:digit:]]a");
        assert_eq!(regex.unwrap().es_valida("hoea").unwrap(), true);
    }

    #[test]
    fn test51_regex_clases09() {
        let regex = Regex::new("ho[[:lower:]]a");
        assert_eq!(regex.unwrap().es_valida("hoRa").unwrap(), false);
    }

    #[test]
    fn test52_regex_clases10() {
        let regex = Regex::new("ho[[:lower:]]a");
        assert_eq!(regex.unwrap().es_valida("hora").unwrap(), true);
    }

    #[test]
    fn test53_regex_clases11() {
        let regex = Regex::new("ho[[:upper:]]a");
        assert_eq!(regex.unwrap().es_valida("hoRa").unwrap(), true);
    }

    #[test]
    fn test54_regex_clases11() {
        let regex = Regex::new("ho[[:upper:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), false);
    }

    #[test]
    fn test55_regex_clases12() {
        let regex = Regex::new("ho[[:space:]]a");
        assert_eq!(regex.unwrap().es_valida("ho a").unwrap(), true);
    }

    #[test]
    fn test56_regex_clases13() {
        let regex = Regex::new("ho[[:space:]]a");
        assert_eq!(regex.unwrap().es_valida("hola").unwrap(), false);
    }

    #[test]
    fn test57_regex_clases14() {
        let regex = Regex::new("ho[[:punct:]]a");
        assert_eq!(regex.unwrap().es_valida("ho;a").unwrap(), true);
    }

    #[test]
    fn test55_regex_clases15() {
        let regex = Regex::new("ho[[:punct:]]a");
        assert_eq!(regex.unwrap().es_valida("ho9a").unwrap(), false);
    }

    #[test]
    fn test56_regex_combinado_clases01() {
        let regex = Regex::new("^ho[[:punct:]]{2}a+");
        assert_eq!(regex.unwrap().es_valida("ho..aaaaaa").unwrap(), true);
    }

    #[test]
    fn test57_regex_combinado_clases01() {
        let regex = Regex::new("ho[[:punct:]]{2}a+");
        assert_eq!(regex.unwrap().es_valida("aaaaa ho..aaaaaa").unwrap(), true);
    }

    #[test]
    fn test55_regex_combinado_clases01() {
        let regex = Regex::new("[a-kA-G]ho[[:punct:]]*a\\.?");
        assert_eq!(regex.unwrap().es_valida("Dho;.a.").unwrap(), true);
    }
}
