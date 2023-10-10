use proc_macro2::{Ident, Span};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Type(pub String);


impl Type {
    pub fn is_string(&self) -> bool {
        self.0 == "string"
    }
    fn convert_ts_to_rust(ty: &str) -> &str {
        match ty {
            "boolean" => "bool",
            "string" => "AttrValue",
            "number" => "f64",
            ty => ty
        }
    }
    pub(crate) fn as_ident(&self) -> Ident {
        let ty = Self::convert_ts_to_rust(&self.0);
        Ident::new(&ty, Span::call_site())
    }
}