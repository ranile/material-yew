pub mod textfield;
pub use textfield::{MatTextField};

pub mod validity_state;
pub use validity_state::{ValidityState};

pub mod text_field_type;
pub use text_field_type::TextFieldType;

pub mod textarea;
pub use textarea::MatTextArea;

pub use web_sys::ValidityState as NativeValidityState;

use std::rc::Rc;
pub type ValidityTransformFn = dyn Fn(String, NativeValidityState) -> ValidityState;

#[derive(Clone)]
pub struct ValidityTransform(
    pub Rc<ValidityTransformFn>
);

impl ValidityTransform {
    pub(crate) fn new<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(func: F) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}
