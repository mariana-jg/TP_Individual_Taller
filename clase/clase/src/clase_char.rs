#[derive(Clone, Debug, PartialEq)]

pub enum ClaseChar {
    Alnum,
    Alpha,
    Digit,
    Lower,
    Upper,
    Space,
    Punct,
    Simple(Vec<char>),
}
