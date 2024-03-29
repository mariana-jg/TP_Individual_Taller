use clase::regex::Regex;

fn main() {

    let regex = Regex::new("ho{3,}la");

    let value = "hoola"; 
    //hayq ue ver si algo de lo que consumi puedo haber consumido antes para lograr llegar a la c

    match regex.unwrap().es_valida(value) {
        Ok(result) => println!("Result: {}" ,result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, world!");
}
