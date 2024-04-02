use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

use crate::errors::Error;

const CANTIDAD_ARGUMENTOS: usize = 3;

///Verifica si se puede procesar el archivo ingresado.
/// - Si se puede, devuelve un vector con las lineas del archivo.
/// - Si no se puede abrir el archivo, devuelve un error de tipo FallaAbrirArchivo.
pub fn puedo_procesar_archivo(args: &str) -> Result<Vec<String>, Error> {
    let archivo = File::open(args);
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

            Ok(lineas)
        }
        Err(_err) => Err(Error::FallaAbrirArchivo),
    }
}

///Verifica si la cantidad de argumentos ingresados es correcta.
fn cantidad_correcta_argumentos(cantidad_argumentos: usize) -> bool {
    cantidad_argumentos == CANTIDAD_ARGUMENTOS
}

///Verifica si la cantidad de argumentos ingresados es correcta.
/// - Si la cantidad de argumentos es correcta, devuelve un vector con los argumentos.
/// - Si la cantidad de argumentos no es correcta, devuelve un error de tipo ArgumentosInvalidos.
/// Una vez que se verifica la cantidad de argumentos, se llama a la función puedo_procesar_archivo.
pub fn verificar_inicio(args: Vec<String>) -> Result<Vec<String>, Error> {
    if !cantidad_correcta_argumentos(args.len()) {
        return Err(Error::ArgumentosInvalidos);
    }

    puedo_procesar_archivo(&args[args.len() - 1])
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test01_cantidad_correcta_argumentos() {
        assert_eq!(cantidad_correcta_argumentos(3), true);
        assert_eq!(cantidad_correcta_argumentos(2), false);
        assert_eq!(cantidad_correcta_argumentos(4), false);
    }

    ///Este test y el que sigue tiene en el primer assert un archivo de prueba
    /// que queda adjuntado en la carpeta del proyecto.
    #[test]
    fn test02_verificar_inicio() {
        assert_eq!(
            verificar_inicio(vec![
                "cargo run".to_string(),
                "abcd".to_string(),
                "/home/mari/Escritorio/tp1_taller/grep_rustico/prueba.txt".to_string()
            ]),
            Ok(vec![
                "Hola".to_string(),
                "esto".to_string(),
                "es".to_string(),
                "una".to_string(),
                "prueba".to_string()
            ])
        );
        assert_eq!(
            verificar_inicio(vec![
                "cargo run".to_string(),
                "abcd".to_string(),
                "prueba.txt".to_string(),
                "prueba2.txt".to_string()
            ]),
            Err(Error::ArgumentosInvalidos)
        );
    }

    #[test]
    fn test03_puedo_procesar_archivo() {
        assert_eq!(
            puedo_procesar_archivo("/home/mari/Escritorio/tp1_taller/grep_rustico/prueba.txt"),
            Ok(vec![
                "Hola".to_string(),
                "esto".to_string(),
                "es".to_string(),
                "una".to_string(),
                "prueba".to_string()
            ])
        );
    }
}
