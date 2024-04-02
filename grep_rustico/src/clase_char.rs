use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
///Representa una clase de caracteres que puede ser alfanumérica,
///alfabética, numérica, minúscula, mayúscula, espacio o signo de puntuación,
///que son las clases que soporta nuestro grep rústico.
///Además, se agrega una clase que representa un conjunto de caracteres simples.
pub enum ClaseChar {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
    Simple(HashSet<char>),
}
