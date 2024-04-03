use std::env;

use grep_rustico::{regex::Regex, verificacion_inicial};

fn main() {
    let args: Vec<String> = env::args().collect();

    let lineas = verificacion_inicial::verificar_inicio(args.clone());

    match lineas {
        Ok(lineas) => {
            for l in lineas {
                match Regex::es_valida_general(&args[1], l.as_str()) {
                    Ok(result) => {
                        if result {
                            println!("{}", l);
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }
        }
        Err(error) => println!("{}", error),
    };
}
