
use clase::regex::Regex;

fn verificar_funcion_or (expresion: &str) -> bool {
    let mut hay_corchete_abierto = false;
    let mut hay_corchete_cerrado = false;
    let mut chars_iter = expresion.chars();
    while let Some(c) = chars_iter.next() {
        if c == '[' {
            hay_corchete_abierto = true;
        } 
        if c == ']' {
            hay_corchete_cerrado = true;
        }
        if hay_corchete_abierto && hay_corchete_cerrado && c == '|' ||
        !hay_corchete_cerrado && !hay_corchete_cerrado && c == '|' {
            return true;
        }
    }
    false
}

fn main() {
    let expresion ="[abc]d[[:alpha:]]|k";
    //let expresion = "abc|de+f";
    let value = "adA";


    if expresion.contains('|') && verificar_funcion_or(expresion) {
        let resultado: Vec<&str> = expresion.split('|').collect();
        let regex1 = Regex::new(resultado[0]);
        let regex2 = Regex::new(resultado[1]);
        let mut result1 = false;
        let mut result2 = false;

        match regex1.unwrap().es_valida(value) {
            Ok(result) => {println!("Result: {}", result);
                result1 = result;},
            Err(err) => println!("Error: {}", err),
        }
      
        match regex2.unwrap().es_valida(value) {
            Ok(result) => {println!("Result: {}", result);
                result2 = result;},
            Err(err) => println!("Error: {}", err),
        }
        if result1 || result2 {
            println!("Result final: {}", true);
        } else {
            println!("Result final: {}", false);
        }

    } else {
        let regex = Regex::new(expresion);
    
        match regex.unwrap().es_valida(value) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }

    println!("Hello, world!");
}
