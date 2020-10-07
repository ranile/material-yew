pub mod textfield;
pub mod validity_state;
pub mod text_field_type;
pub mod textarea;

pub use textfield::{MatTextField, NativeValidityState};
pub use validity_state::{ValidityState};
pub use validity_state::{ValidityStateJS};
pub use text_field_type::TextFieldType;
pub use textarea::MatTextArea;

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
