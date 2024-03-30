use clase::regex::Regex;

fn main() {
    let regex = Regex::new("ho[abc]a");

    let value = "hoaa";

    match regex.unwrap().es_valida(value) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, world!");
}
