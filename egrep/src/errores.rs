use std::fmt;

pub enum Error {
    FallaAbrirArchivo,
    FallaLecturaArchivo,
    ArgumentosInvalidos,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FallaAbrirArchivo => write!(f, "Error: No se pudo abrir el archivo."),
            Error::FallaLecturaArchivo => write!(f, "Error: No se pudo leer el archivo."),
            Error::ArgumentosInvalidos => write!(f, "Error: La cantidad de argumentos no es válida."),
        }
    }
}