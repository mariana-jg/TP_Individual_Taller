use std::env::{self, args};

use errors::Error;

mod verficacion_inicial;

fn main() {

    let args: Vec<String> = env::args().collect();

    let lineas = verficacion_inicial::verificar_inicio(args.clone());

    match lineas {
        Ok(lineas) => {
            println!("tu regex es: {}", args[1]);

            for l in lineas{  
                println!("{}", l);
            }
        },
        Err(error) =>  println!("{}", error),

    };      
}
