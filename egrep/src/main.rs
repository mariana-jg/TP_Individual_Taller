use std::env;

extern crate errores;

mod verficacion_inicial;

fn main() {

    let args: Vec<String> = env::args().collect();

    let lineas = verficacion_inicial::verificar_inicio(args);

    match lineas {
        Ok(lineas) => {
            for l in lineas{  
                println!("{}", l);
            }
        },
        Err(error) =>  println!("{}", error),

    };      
}
