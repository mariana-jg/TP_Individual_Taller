use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use errors::Error;

use crate::errors;

pub fn puedo_procesar_archivo(args: &str) -> Result<Vec<String>, Error> {
    let archivo = File::open(args.to_string());
    match archivo {
        Ok(archivo) => {
            let mut lineas: Vec<String> = vec![];
            let reader: Lines<BufReader<&File>> = BufReader::new(&archivo).lines();
            for linea in reader {
                match linea {
                    Ok(linea) => lineas.push(linea),
                    Err(_err) => return Err(Error::FallaLecturaArchivo),
                };
            }

            return Ok(lineas);
        }
        Err(_err) => return Err(Error::FallaAbrirArchivo),
    };
}

fn cantidad_correcta_argumentos(cantidad_argumentos: usize) -> bool {
    return cantidad_argumentos == 3;
}

pub fn verificar_inicio(args: Vec<String>) -> Result<Vec<String>, Error> {
    if !cantidad_correcta_argumentos(args.len()) {
        return Err(Error::ArgumentosInvalidos);
    }

    match puedo_procesar_archivo(&args[args.len() - 1]) {
        Ok(lineas) => return Ok(lineas),
        Err(error) => return Err(error),
    }
}
