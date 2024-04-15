use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
///Representa una clase de caracteres que puede ser alfanumérica,
///alfabética, numérica, minúscula, mayúscula, espacio o signo de puntuación,
///que son las clases que soporta nuestro grep rústico.
///Además, se agrega una clase que representa un conjunto de caracteres simples.
pub enum ClaseChar {
    Alnum(bool),
    Alpha(bool),
    Digit(bool),
    Lower(bool),
    Upper(bool),
    Space(bool),
    Punct(bool),
    Simple(HashSet<char>, bool),
}
