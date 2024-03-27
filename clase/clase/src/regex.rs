use std::collections::VecDeque;

use crate::step_regex::StepRegex;
use crate::caracter::Caracter;
use crate::repeticion::Repeticion;
use crate::step_evaluado::StepEvaluado;
use crate::errors::Error;

pub struct Regex {
    pasos: Vec<StepRegex>
}

impl Regex {

    pub fn new(expression: &str) -> Result<Self, Error> {

        let mut steps: Vec<StepRegex> = Vec::new();
        let mut chars_iter = expression.chars();

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
                

               /*  '&' => {
                    if let Some(last) = steps.last_mut() {
                        last.repeticiones = Repeticion::Rango { min: Some(3), max: Some(4) };
                    } else {
                        return Err(Error::CaracterNoProcesable);
                    }
                    None
                },  */


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
        
        Ok (Regex { pasos: steps })
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
                    for _ in [0..n] {
                        let avance = paso.caracter_interno.coincide(&linea[index..]);

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
                        match_size,
                        backtrackable: false
                    })
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
                Repeticion::Rango { min, max } => 
                    return Ok(false),
            }
        }
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
        assert_eq!(regex.unwrap().es_valida("abcccccdefg").unwrap(), false);
    }

    #[test]
    fn test03_regex_con_asterisk() {
        let regex = Regex::new("ab*c");
        assert_eq!(regex.unwrap().es_valida("abcccccdefcg").unwrap(), true);
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

}