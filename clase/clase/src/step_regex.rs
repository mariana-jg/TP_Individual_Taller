use crate::caracter::Caracter;
use crate::repeticion::Repeticion;


#[derive(Clone, Debug)]

pub struct StepRegex {
    pub(crate) caracter_interno: Caracter, 
    pub(crate) repeticiones: Repeticion,
}