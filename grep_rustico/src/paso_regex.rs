pub(crate) use crate::caracter::Caracter;
use crate::repeticion::Repeticion;

#[derive(Clone, Debug)]
///Representa un paso de la expresión regular que puede ser un caracter interno y una repetición.
///El caracter interno es un caracter que se espera que sea exactamente igual al que se está comparando.
///La repetición es la cantidad de veces que se espera que se repita el caracter interno.
pub struct PasoRegex {
    pub(crate) caracter_interno: Caracter,
    pub(crate) repeticiones: Repeticion,
}
