use clase::regex::Regex;

fn main() {
    let regex = Regex::new("ho[xsd]a");

    let value = "hoxa";

    match regex.unwrap().es_valida(value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, world!");
}
