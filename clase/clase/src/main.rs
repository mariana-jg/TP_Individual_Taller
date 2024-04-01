use clase::regex::{self, Regex};

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
    //let expresion ="[abc]d[[:alpha:]]|k";
    let expresion = "[[:digit:]]|de+f";
    let value = "hola  abc ";

    //let regex = Regex::new(expresion);

    match Regex::es_valida_general(expresion,value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    println!("Hello, world!");
}
