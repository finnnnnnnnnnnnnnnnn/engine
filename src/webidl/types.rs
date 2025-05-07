use imstr::ImString;

pub trait IsDOMString {
    fn get_domstring(&self) -> ImString;
}

#[derive(Clone)]
pub struct DOMString {
    dom_string: ImString
}

impl IsDOMString for DOMString {
    fn get_domstring(&self) -> ImString {
        // bad performance????
        self.dom_string.clone()
    }
}

pub type OptionalDOMString = Option<DOMString>;


use scalar_value_string::SvString;

pub type USVString = SvString;