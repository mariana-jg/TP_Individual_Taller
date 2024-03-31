#[derive(Clone, Debug)]

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
