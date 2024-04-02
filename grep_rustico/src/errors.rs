use std::fmt;

#[derive(Debug, PartialEq)]
///Crater de errores personalizados para casos específicos.
///Cada uno con un mensaje que le dará al usuario una idea de lo que salió mal.
pub enum Error {
    FallaAbrirArchivo,
    FallaLecturaArchivo,
    ArgumentosInvalidos,
    CaracterNoProcesable,
    FormatoDeLineaNoASCII,
    ErrorEnLlaves,
    ErrorEnCorchetes,
    ErrorEnRepeticion,
    ErrorEnFuncionOR,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FallaAbrirArchivo => write!(f, "Error: No se pudo abrir el archivo."),
            Error::FallaLecturaArchivo => write!(f, "Error: No se pudo leer el archivo."),
            Error::ArgumentosInvalidos => {
                write!(f, "Error: La cantidad de argumentos no es válida.")
            }
            Error::CaracterNoProcesable => {
                write!(f, "Error: Ingresaste un caracter no posible de procesar.")
            }
            Error::FormatoDeLineaNoASCII => {
                write!(f, "Error: La linea ingresada no esta en formato ASCII.")
            }
            Error::ErrorEnLlaves => {
                write!(
                    f,
                    "Error: No se cumple con el formato para el uso de las llaves correctamente."
                )
            }
            Error::ErrorEnCorchetes => {
                write!(f, "Error: No se cumple con el formato para el uso de los corchetes correctamente.")
            }
            Error::ErrorEnRepeticion => {
                write!(f, "Error: No se cumple con el formato para el uso de las repeticiones correctamente.")
            }
            Error::ErrorEnFuncionOR => {
                write!(f, "Error: No se cumple con el formato para el uso de la funcion OR correctamente.")
            }
        }
    }
}
