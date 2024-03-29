use std::collections::VecDeque;
use std::f32::consts::E;
use std::str::Chars;

use crate::step_regex::StepRegex;
use crate::caracter::Caracter;
use crate::repeticion::Repeticion;
use crate::step_evaluado::StepEvaluado;
use crate::errors::Error;


pub struct Regex {
    pasos: Vec<StepRegex>,
    //anchorings: Anchoring,
}

pub fn agregar_pasos(steps: &mut Vec<StepRegex>,  chars_iter: &mut Chars<'_>) -> Result<Vec<StepRegex>, Error>{

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
                    while let Some(c) = chars_iter.next() {
                        if c == '}' {
                            break;
                        } else if c.is_digit(10) {
                            match c.to_digit(10) {
                                Some(cant) =>
                                    last.repeticiones = Repeticion::Exacta(cant as usize),
                                   // last.repeticiones = Repeticion::Rango{min: Some(cant as usize), max: None};

                                None => return Err(Error::CaracterNoProcesable),
                            }
                        } else {
                            return Err(Error::CaracterNoProcesable);
                        }
                    }
                        
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            },

            '?' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Rango { min: Some(0), max: Some(1) };
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            },   

            '*' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Alguna;
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            },   

            '+' => {
                if let Some(last) = steps.last_mut() {
                    last.repeticiones = Repeticion::Rango { min: Some(1), max: None };
                } else {
                    return Err(Error::CaracterNoProcesable);
                }
                None
            }, 

            '\\' => match chars_iter.next() {
                Some(literal) => Some(StepRegex {
                    repeticiones: Repeticion::Exacta(1),
                    caracter_interno: Caracter::Literal(literal),
                }),
                None => return Err(Error::CaracterNoProcesable)
            },
            
            _ => return Err(Error::CaracterNoProcesable),
        
        };

        if let Some(p) = step {
            steps.push(p);
        }
    };

    Ok(steps.to_vec())
}

impl Regex {

    pub fn new(expression: &str) -> Result<Self, Error> {

        let mut steps: Vec<StepRegex> = Vec::new();
        let mut chars_iter = expression.chars();


        /*if !expression.starts_with('^') {
                    let paso = Some(StepRegex {
            repeticiones: Repeticion::Exacta(1),
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
        
        chars_iter.next();  */

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
        Ok (Regex { pasos: steps})
    }


    pub fn es_valida(self, linea: &str) -> Result<bool, Error> {
        if !linea.is_ascii() {
            return Err(Error::FormatoDeLineaNoASCII);
        }
//tengo todos los pasos que quiero hacer, la idea es una vez que termine
//todos los pasos pongo uno adicional que sea salto de linea
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
                                None => {
                                    return Ok(false)
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
                },
                Repeticion::Alguna => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            stack.push(StepEvaluado {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackable: true
                            })
                        } else {
                            sigo_avanzando = false;
                        }
                    }
                },
                Repeticion::Rango { min, max } => {
                    println!("{}", "holaaa");
                    let min = match min {
                        Some(min) => min,
                        None => 0,
                    };
                    
                    let max = match max {
                        Some(max) => max,
                        None => linea.len() - index, 
                    };   

                    let mut match_size = 0;
                    for _ in min..max {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);
                        println!("hay coincidencia: {:?}", paso.caracter_interno);
                        if avance == 0 {
                            
                            match backtrack(paso, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    continue 'pasos;
                                }
                                None => {
                                    return Ok(false)
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

                },
                    

            }}
        Ok(true)
    }

}

//como funciona backtrack?
fn backtrack (current: StepRegex, evaluated: &mut Vec<StepEvaluado>, next: &mut VecDeque<StepRegex>,)
    -> Option<usize> {
        let mut back_size =  0; 
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
        assert_eq!(regex.unwrap().es_valida("abeabdccccccfcg").unwrap(), true);
    }

    #[test]
    fn test04_regex_con_metacaracter_con_backlash() {
        let regex = Regex::new("a\\*");
        assert_eq!(regex.unwrap().es_valida("a*cds").unwrap(), true);
    }

    #[test]
    fn test05_regex_con_repeticiones_dentro_de_rango() {
        let regex = Regex::new("aaa+");
        assert_eq!(regex.unwrap().es_valida("pa").unwrap(), true);
    }

    #[test]
    fn test05_regex_con_anchoring_caret() {
        let regex = Regex::new("^hola");
        assert_eq!(regex.unwrap().es_valida("holaestas").unwrap(), true);
    }

    #[test]
    fn test05_regex_con_anchoring_dollar() {
        let regex = Regex::new("a{2}");
        assert_eq!(regex.unwrap().es_valida("a").unwrap(), false);
    }

    #[test]
    fn test06_regex_con_anchoring_() {
        let regex = Regex::new("ba{2}");
        assert_eq!(regex.unwrap().es_valida("aa").unwrap(), false);
    }

    #[test]
    fn test07_regex_con_anchoring_() {
        let regex = Regex::new("ba{2}c");
        assert_eq!(regex.unwrap().es_valida("bac").unwrap(), false);
    }

}