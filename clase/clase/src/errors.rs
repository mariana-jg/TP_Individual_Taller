use std::fmt;

#[derive(Debug)]
pub enum Error {
    FallaAbrirArchivo,
    FallaLecturaArchivo,
    ArgumentosInvalidos,
    CaracterNoProcesable,
    FormatoDeLineaNoASCII,
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
        }
    }
}
