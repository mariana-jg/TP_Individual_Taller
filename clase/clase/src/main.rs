use clase::regex::Regex;

fn main() {
    let regex = Regex::new("ho[lmn]a");

    let value = "hola";

    match regex.unwrap().es_valida(value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, world!");
}
