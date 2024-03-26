use std::collections::VecDeque;

#[derive(Clone)]
enum RegexRep {
    Any, //no me importa la cantidad de veces que se repite
    Exact(usize), //si quiero que aparezca determinada cantidad de veces
    //Range(Option(usize), Option(usize)), // si quiero que aparezca entre un valor y otro, el option es por si alguno de los 2 no aparece
    /*Range {
        min: Option<usize>,
        max: Option<usize>,
    },*/
}

#[derive(Clone)]
enum RegexVal {
    Literal(char), //se que tengo el valor con el que quiero matchear
    Wildcard, //comodin
    //Clase(RegexClase), se pueden definir todas las variantes de las clases en otro enum
}

#[derive(Clone)]
pub struct RegexStep { //contenido de cada paso
    val: RegexVal, //lo que hay que matchear
    rep: RegexRep, //las veces que matchea
}

pub struct Regex {
    //se que es una secuencia de cosas, podemos pensarla como un vector, contendra los casos que quiero
    //evaluar
    steps: Vec<RegexStep>
}

pub struct EvaluatedStep {
    paso: RegexStep,
    match_size: usize,
    backtrackable: bool
}




impl RegexVal {
    pub fn matches (&self, value: &str) -> usize {
        //deberia devolver usize ya que quisiera devolver
        //cuanto coinsumi del input cuando matchee
        match self {
            RegexVal::Literal(l) => {
                if value.chars().next() == Some(*l) {
                    l.len_utf8() //cantidad consumida en el input
                } else {
                    0 
                }
            },
            RegexVal::Wildcard => {
                if let Some(c) = value.chars().next() {
                    c.len_utf8() //cantidad consumida en el input
                } else {
                    0 
                }
            },
        }
    }
}



//Tenemos nuestra regular expression que es un vector de pasos
//y cada paso tiene un valor y una cantidad de veces que se repite

//cada enum deberia tener un archivo aparte

//el main debes ser lo mas chico posible

//el tipo &str no se puede indexar con un integer porque no todos los caracteres ocupan lo mismo

impl Regex {
    //expression: slice de str
    //result me pide Result<cosa 1, cosa 2>, tengo que devolver Ok(cosa 1 ) o Err(cosa 2)
    pub fn new(expression: &str) -> Result<Self, &str> {
        //recorrer el string y crear nuestra Regex
        let mut steps: Vec<RegexStep> = Vec::new();
        let mut chars_iter = expression.chars(); //voy recorriendo en la medida que voy necesitando
        //expression.chars().for_each()//si uso el for each no me va a servir por ejemplo para {} []
        while let Some(c) = chars_iter.next() {
            let step = match c {
            
                '.' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Wildcard,
                }),
                
                'a'..='z' => Some(RegexStep {
                    rep: RegexRep::Exact(1),
                    val: RegexVal::Literal(c),
                }),    
                
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Any;
                    } else {
                        return Err("* invalido");
                    }
                    None
                },   

                '\\' => match chars_iter.next() {
                    Some(literal) => Some(RegexStep {
                        rep: RegexRep::Exact(1),
                        val: RegexVal::Literal(literal),
                    }),
                    None => return Err("Caracter no valido.")
                },
                
                _ => return Err("Caracter no valido."),
            
            };

            if let Some(p) = step {
                steps.push(p);
            }
        };
        
        Ok (Regex { steps })
    }


    pub fn test(self, linea: &str) -> Result<bool, &str> {
        if !linea.is_ascii() {
            return Err("No es ASCII");
        }

        let mut queue: VecDeque<RegexStep> = VecDeque::from(self.steps);
        let mut stack: Vec<EvaluatedStep> = Vec::new();
        let mut index = 0;

        'pasos: while let Some(paso) = queue.pop_front() {
            match paso.rep {
                RegexRep::Exact(n) => {
                    let mut match_size = 0;

                    for _ in [0..n] {
                        let avance = paso.val.matches(&linea[index..]);

                        if avance == 0 { //puedo encontrar la coincidencia mas adelante
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

                    stack.push(EvaluatedStep {
                        paso: paso,
                        match_size,
                        backtrackable: false
                    })
                },
                RegexRep::Any => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {
                        let avance = paso.val.matches(&linea[index..]);

                        if avance != 0 {
                            index += avance;
                            stack.push(EvaluatedStep {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackable: true
                            })
                        } else {
                            sigo_avanzando = false;
                        }

                    }
                },

            }
        }

        Ok(true)
    }

}

fn backtrack (current : RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,) -> Option<usize> {
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

//creo el regex step y lo agrego al vector
//para cada caracter voy a querer crear un paso
//no se cuantas veces voy a matchar, empiezo con 1 y si viene algo de repeticion lo voy a matchear 1
// en el ejemplo el infiere que empieza con el anchoring