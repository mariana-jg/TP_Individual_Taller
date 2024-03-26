use crate::caracter::Caracter;
use crate::repeticion::Repeticion;

#[derive(Clone)]

pub struct StepRegex { //contenido de cada paso
    pub(crate) caracter_interno: Caracter, //lo que hay que matchear
    pub(crate) repeticiones: Repeticion, //las veces que matchea
}