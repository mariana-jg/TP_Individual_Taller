#[derive(Clone, Debug, PartialEq)]
///Representa una repetición que puede ser exacta, alguna o un rango.
/// - Exacta: se espera que se repita exactamente la cantidad de veces indicada.
/// - Alguna: se espera que se repita alguna vez.
/// - Rango: se espera que se repita una cantidad de veces dentro de un rango, donde
///se guarda el mínimo y máximo de repeticiones.
/// Además, las repeticiones Exacta y Alguna cuentan con un booleano que indica 
/// si la repetición es negada o no. Esto nos sirve para saber si se espera que
/// se repita exactamente la cantidad de veces indicada o no se espera que se repita, 
/// como por ejemplo en los casos del operador de negación ^ dentro de los corchetes.
pub enum Repeticion {
    Exacta(usize, bool),
    Alguna(bool),
    Rango {
        min: Option<usize>,
        max: Option<usize>,
    },
}
