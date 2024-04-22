use std::collections::{HashSet, VecDeque};
use std::str::Chars;

use crate::caracter::Caracter;
use crate::clase_char::ClaseChar;
use crate::errors::Error;
use crate::paso_evaluado::PasoEvaluado;
use crate::paso_regex::PasoRegex;
use crate::repeticion::Repeticion;

///Caracteres especiales que se utilizan en las expresiones regulares.
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

///Representa una expresión regular que se puede evaluar en una cadena de texto.
/// Contiene una lista de pasos que se deben cumplir para que la expresión regular sea válida.
pub struct Regex {
    pasos: Vec<PasoRegex>,
}

///Obtiene el contenido de un corchete, si es que lo hay.
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

///Determina el contenido de un corchete.
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

///Obtiene la clase de caracter que se debe evaluar.
/// - Si la clase de caracter es una de las predefinidas, se devuelve la clase de caracter correspondiente.
/// - Si la clase de caracter no es predefinida, se determina el contenido de la clase de caracter.
/// - Si no se puede determinar el contenido de la clase de caracter, se devuelve un error.
fn conseguir_lista(chars_iter: &mut Chars<'_>) -> Result<ClaseChar, Error> {
    let (auxiliar, hay_clase, es_negado) = obtener_auxiliar(chars_iter);

    if hay_clase {
        let class: String = auxiliar.iter().collect();

        match class.to_string().as_str() {
            "alpha" => return Ok(ClaseChar::Alpha(es_negado)),
            "alnum" => return Ok(ClaseChar::Alnum(es_negado)),
            "digit" => return Ok(ClaseChar::Digit(es_negado)),
            "lower" => return Ok(ClaseChar::Lower(es_negado)),
            "upper" => return Ok(ClaseChar::Upper(es_negado)),
            "space" => return Ok(ClaseChar::Space(es_negado)),
            "punct" => return Ok(ClaseChar::Punct(es_negado)),
            _ => {}
        }
    }

    let contenido = determinar_contenido_a_evaluar(auxiliar);

    match contenido {
        Ok(contenido) => Ok(ClaseChar::Simple(contenido, es_negado)),
        Err(error) => Err(error),
    }
}

///Determina si el caracter anterior es un comodín.
///Para verificar que la expresión regular no comience con una repetición.
fn no_hay_anterior(anterior: &mut PasoRegex) -> bool {
    anterior.caracter_interno == Caracter::Comodin && anterior.repeticiones == Repeticion::Alguna
}

fn fabricar_paso_punto() -> Result<Option<PasoRegex>, Error> {
    Ok(Some(PasoRegex {
        repeticiones: Repeticion::Exacta(1),
        caracter_interno: Caracter::Comodin,
    }))
}

fn fabricar_paso_literal(c: char) -> Result<Option<PasoRegex>, Error> {
    Ok(Some(PasoRegex {
        repeticiones: Repeticion::Exacta(1),
        caracter_interno: Caracter::Literal(c),
    }))
}

fn fabricar_paso_llave(
    steps: &mut [PasoRegex],
    chars_iter: &mut Chars<'_>,
) -> Result<Option<PasoRegex>, Error> {
    if let Some(ultimo) = steps.last_mut() {
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
                ultimo.repeticiones = Repeticion::Rango {
                    min: None,
                    max: Some(rangos[0]),
                };
            } else if contenido[contenido.len() - 1] == ',' {
                ultimo.repeticiones = Repeticion::Rango {
                    min: Some(rangos[0]),
                    max: None,
                };
            } else {
                if rangos[0] > rangos[1] {
                    return Err(Error::ErrorEnLlaves);
                }
                ultimo.repeticiones = Repeticion::Rango {
                    min: Some(rangos[0]),
                    max: Some(rangos[1]),
                };
            }
        } else if contenido.len() == 1 && contenido[0].is_ascii_digit() {
            ultimo.repeticiones = Repeticion::Exacta(rangos[0]);
        } else {
            return Err(Error::ErrorEnLlaves);
        }
    }
    Ok(None)
}

fn fabricar_paso_corchete(chars_iter: &mut Chars<'_>) -> Result<Option<PasoRegex>, Error> {
    match conseguir_lista(chars_iter) {
        Ok(contenido) => Ok(Some(PasoRegex {
            repeticiones: Repeticion::Exacta(1),
            caracter_interno: Caracter::Serie(contenido),
        })),
        Err(error) => Err(error),
    }
}

fn fabricar_paso_interrogacion(steps: &mut [PasoRegex]) -> Result<Option<PasoRegex>, Error> {
    if let Some(ultimo) = steps.last_mut() {
        if no_hay_anterior(ultimo) {
            return Err(Error::ErrorEnRepeticion);
        } else {
            ultimo.repeticiones = Repeticion::Rango {
                min: Some(0),
                max: Some(1),
            };
        }
    }
    Ok(None)
}

fn fabricar_paso_mas(steps: &mut [PasoRegex]) -> Result<Option<PasoRegex>, Error> {
    if let Some(ultimo) = steps.last_mut() {
        if no_hay_anterior(ultimo) {
            return Err(Error::ErrorEnRepeticion);
        } else {
            ultimo.repeticiones = Repeticion::Rango {
                min: Some(1),
                max: None,
            };
        }
    }
    Ok(None)
}

fn fabricar_paso_asterisco(
    steps: &mut [PasoRegex],
) -> Result<Option<PasoRegex>, Error> {
    if let Some(ultimo) = steps.last_mut() {
        if no_hay_anterior(ultimo) {
            return Err(Error::ErrorEnRepeticion);
        } else {
            ultimo.repeticiones = Repeticion::Alguna;
        }
    }
    Ok(None)
}

fn fabricar_paso_barra(chars_iter: &mut Chars<'_>) -> Result<Option<PasoRegex>, Error> {
    match chars_iter.next() {
        Some(literal) => Ok(Some(PasoRegex {
            repeticiones: Repeticion::Exacta(1),
            caracter_interno: Caracter::Literal(literal),
        })),
        None => Err(Error::CaracterNoProcesable),
    }
}

fn fabricar_paso_dolar() -> Result<Option<PasoRegex>, Error> {
    Ok(Some(PasoRegex {
        repeticiones: Repeticion::Exacta(1),
        caracter_interno: Caracter::Dolar,
    }))
}

fn fabricar_paso_caracter(
    c: char,
    steps: &mut [PasoRegex],
    chars_iter: &mut Chars<'_>,
) -> Result<Option<PasoRegex>, Error> {
    match c {
        PUNTO => fabricar_paso_punto(),
        'a'..='z' | 'A'..='Z' | '0'..='9' | ' ' => fabricar_paso_literal(c),
        LLAVE_ABIERTA => fabricar_paso_llave(steps, chars_iter),
        CORCHETE_ABIERTO => fabricar_paso_corchete(chars_iter),
        INTERROGACION => fabricar_paso_interrogacion(steps),
        ASTERISCO => fabricar_paso_asterisco(steps),
        MAS => fabricar_paso_mas(steps),
        BARRA => fabricar_paso_barra(chars_iter),
        DOLAR => fabricar_paso_dolar(),
        CARET => Ok(None),
        FUNCION_OR => Ok(None),
        _ => Err(Error::CaracterNoProcesable),
    }
}

///Agrega los pasos a la expresión regular.
/// - Si el caracter es un punto, se agrega un paso con un comodín.
/// - Si el caracter es un literal, se agrega un paso con el literal.
/// - Si el caracter es una llave abierta, se obtiene el contenido de la llave y se agrega un paso con la cantidad de repeticiones
/// y según si se trata de una repetición exacta, con solo mínimo, con solo máximo o ambas.
/// - Si el caracter es un corchete abierto, se obtiene el contenido del corchete y se agrega un paso con la clase de caracteres.
/// - Si el caracter es un asterisco, se agrega un paso con Alguna cantidad de repeticiones.
/// - Si el caracter es un signo de interrogación, se agrega un paso con la cantidad de repeticiones (0 o 1 vez).
/// - Si el caracter es un signo de más, se agrega un paso con la cantidad de repeticiones (1 o más).
/// - Si el caracter es una barra, se obtiene el siguiente caracter y se agrega un paso con el literal.
/// - Si el caracter es un dolar, se agrega un paso con un dolar.
/// - Si el caracter es un caret, no se agrega un paso.
/// - Si el caracter es una función OR, no se agrega un paso.
/// - Si el caracter no es procesable, se devuelve un error.
pub fn agregar_pasos(
    pasos: &mut Vec<PasoRegex>,
    chars_iter: &mut Chars<'_>,
) -> Result<Vec<PasoRegex>, Error> {
    while let Some(c) = chars_iter.next() {
        if let Some(paso) = fabricar_paso_caracter(c, pasos, chars_iter)? {
            pasos.push(paso);
        }
    }
    Ok(pasos.to_vec())
}

///Determina si se debe agregar un paso con un comodín al principio de la expresión regular
///(como un .*), si esta no comienza con un CARET ^.
fn definir_uso_de_caret(expresion: &str, pasos: &mut Vec<PasoRegex>) {
    if !expresion.starts_with(CARET) {
        let paso = Some(PasoRegex {
            repeticiones: Repeticion::Alguna,
            caracter_interno: Caracter::Comodin,
        });
        if let Some(p) = paso {
            pasos.push(p);
        }
    }
}

///Verifica si la expresión regular está escrita correctamente.
///Determina si las llaves y los corchetes se abren y se cierran
///como corresponde. En cada caso devuelve un error explicativo.
///Además, verifica que la función OR no esté dentro de llaves o corchetes.
fn expresion_escrita_correctamente(expresion: &str) -> Result<(), Error> {
    let iter = expresion.chars();
    let mut cont_llaves = 0;
    let mut cont_corchetes = 0;
    for c in iter {
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
    ///Verifica si una expresión regular es válida para una línea de texto.
    /// - Si la expresión regular está escrita correctamente, se evalúa si la línea cumple con la expresión regular.
    /// En caso de tener una función OR, se evalúa si alguna de las expresiones es válida.
    /// - Si la expresión regular no está escrita correctamente, se devuelve un error.
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
            Err(error) => Err(error),
        }
    }

    ///Crea una nueva expresión regular a partir de una cadena de texto.
    ///Teniendo en cuenta si comienza con un CARET ^ o no.
    pub fn new(expresion: &str) -> Result<Self, Error> {
        let mut pasos: Vec<PasoRegex> = Vec::new();
        let mut chars_iter = expresion.chars();

        definir_uso_de_caret(expresion, &mut pasos);

        let pasos: Vec<PasoRegex> = agregar_pasos(&mut pasos, &mut chars_iter)?;
        Ok(Regex { pasos })
    }

    ///Procesa "alguna" repeticion.
    fn procesar_alguna(
        paso: &PasoRegex,
        linea: &str,
        index: &mut usize,
        pila: &mut Vec<PasoEvaluado>,
    ) {
        let mut sigo_avanzando = true;
        while sigo_avanzando {
            let avance = paso.caracter_interno.coincide(&linea[*index..]);
            if avance != 0 {
                *index += avance;
                pila.push(PasoEvaluado {
                    paso: paso.clone(),
                    tam_matcheo: avance,
                    backtrackeable: true,
                })
            } else {
                sigo_avanzando = false;
            }
        }
    }

    ///Procesa una repetición exacta.
    fn procesar_exacta(
        cola: &mut VecDeque<PasoRegex>,
        pila: &mut Vec<PasoEvaluado>,
        index: &mut usize,
        paso: PasoRegex,
        linea: &str,
        n: usize,
    ) -> Result<bool, Error> {
        let mut tam_coincidencia = 0;
        for _ in 0..n {
            let avance = paso.caracter_interno.coincide(&linea[*index..]);
            if avance == 0 {
                if let Some(size) = backtrack(paso.clone(), pila, cola) {
                    *index -= size;
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            } else {
                tam_coincidencia += avance;
                *index += avance;
            }
        }
        pila.push(PasoEvaluado {
            paso,
            tam_matcheo: tam_coincidencia,
            backtrackeable: false,
        });
        Ok(true)
    }

   /*fn procesar_rango(
        cola: &mut VecDeque<PasoRegex>,
        pila: &mut Vec<PasoEvaluado>,
        index: &mut usize,
        paso: PasoRegex,
        linea: &str,
        min: Option<usize>,
        max: Option<usize>,
    ) -> Result<bool, Error> {
        let min = min.unwrap_or(0);

        let max = match max {
            Some(max) => max,
            None => linea.len() - *index,
        };
        let mut matches = 0;
        let mut sigo_avanzando = true;
        while sigo_avanzando {
            let avance = paso.caracter_interno.coincide(&linea[*index..]);

            if avance != 0 {
                matches += 1;
                let back = matches >= min;
                *index += avance;
                pila.push(PasoEvaluado {
                    paso: PasoRegex { caracter_interno: paso.caracter_interno.clone(), 
                        repeticiones: Repeticion::Exacta(1) },
                    tam_matcheo: avance,
                    backtrackeable: back,
                });
                if matches == max || *index == linea.len() {
                    sigo_avanzando = false;
                }
            } else {
                sigo_avanzando = false;
            }
        }

        if matches < min {
            return Ok(false);
        }

        Ok(true)
    }
*/

fn procesar_rango(
    cola: &mut VecDeque<PasoRegex>,
    pila: &mut Vec<PasoEvaluado>,
    index: &mut usize,
    paso: PasoRegex,
    linea: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<bool, Error> {
    let min = min.unwrap_or(0);

    let max = match max {
        Some(max) => max,
        None => linea.len() - *index,
    };
    let mut matches = 0;
  //  let mut backtrack_size = 0;
    let mut sigo_avanzando = true;
    while sigo_avanzando {
        let avance = paso.caracter_interno.coincide(&linea[*index..]);

        if avance != 0 {
            matches += 1;
            let back = matches >= min && matches <= max;
          //  backtrack_size += avance;
            /*  if back {
                if let Some(size) = backtrack(paso.clone(), pila, cola) {
                    *index -= size;
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }*/
            *index += avance;
            pila.push(PasoEvaluado {
                paso: PasoRegex {
                    caracter_interno: paso.caracter_interno.clone(),
                    repeticiones: Repeticion::Exacta(1),
                },
                tam_matcheo: avance,
                backtrackeable: back,
            });
            if matches == max || *index == linea.len() {
                sigo_avanzando = false;
            }
        } else {
            sigo_avanzando = false;
        }
    }

    if matches < min {
        if let Some(size) = backtrack(paso.clone(), pila, cola) {
            *index -= size;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    Ok(true)
}

    ///Verifica si una expresión regular es válida para una línea de texto,
    ///es el "validador" de la expresión regular.
    ///Según el tipo de repetición, se busca en la línea de texto la
    ///coincidencia. Si no se encuentra, se evalúa si se puede hacer un backtrack.
    pub fn es_valida(self, linea: &str) -> Result<bool, Error> {
        if !linea.is_ascii() {
            return Err(Error::FormatoDeLineaNoASCII);
        }
        let mut cola: VecDeque<PasoRegex> = VecDeque::from(self.pasos);
        let mut pila: Vec<PasoEvaluado> = Vec::new();
        let mut index = 0;

        while let Some(paso) = cola.pop_front() {
            match paso.repeticiones {
                Repeticion::Exacta(n) => {
                    if !Self::procesar_exacta(&mut cola, &mut pila, &mut index, paso, linea, n)? {
                        return Ok(false);
                    }
                }
                Repeticion::Alguna => Self::procesar_alguna(&paso, linea, &mut index, &mut pila),
                Repeticion::Rango { min, max } => {
                    if !Self::procesar_rango(&mut cola,&mut pila, &mut index, paso, linea, min, max)? {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }
}


///Realiza un backtrack en la expresión regular.
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(regex.unwrap().es_valida("holaaaaa").unwrap(), true);
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
        let regex = Regex::new("ba{2,3}c");
        assert_eq!(regex.unwrap().es_valida("bac baac baaac").unwrap(), true);
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
        assert_eq!(regex.unwrap().es_valida("baaaaaac").unwrap(), true);
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
        assert_eq!(regex.unwrap().es_valida("esa hoa").unwrap(), true);
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
    #[test]
    fn test87_rangos_seguidos() {
        assert_eq!(
            Regex::es_valida_general("abc{2,5}d abc{1,4}d", "en medio abcccd abcd fin").unwrap(),
            true
        );
    }

    #[test]
    fn test88_rangos_seguidos() {
        assert_eq!(
            Regex::es_valida_general("abc{2,5}d abc{0,}d", "en medio abcccccd abd fin").unwrap(),
            true
        );
    }

    #[test]
    fn test89_rangos_seguidos() {
        assert_eq!(
            Regex::es_valida_general("abc{2,5}d abc{0,}d", "en medio abcccd abcd fin").unwrap(),
            true
        );
    }

    #[test]
    fn test90_punto_question() {
        assert_eq!(
            Regex::es_valida_general("ab.?d", "abhhd").unwrap(),
            false
        );
    }

    #[test]
    fn test91_punto_question() {
        assert_eq!(
            Regex::es_valida_general("ab.?d", "hola abcd chau").unwrap(),
            true
        );
    }
}
