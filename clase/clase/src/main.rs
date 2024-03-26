use clase::regex::Regex;

fn main() {

    let regex = Regex::new(
        "ab.*c"
    );

    let value = "abccccccd"; 
    //hayq ue ver si algo de lo que consumi puedo haber consumido antes para lograr llegar a la c

    match regex.unwrap().test(value) {
        Ok(result) => println!("Result: {}" ,result),
        Err(err) => println!("Error: {}", err),
    }
    println!("Hello, world!");
}
