#[derive(Clone, Debug)]
#[derive(PartialEq)]

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
