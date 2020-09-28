pub mod textfield;
pub mod validity_state;
pub mod text_field_type;
pub mod textarea;

pub use textfield::{MatTextField, NativeValidityState};
pub use validity_state::{ValidityState};
pub use text_field_type::TextFieldType;
pub use textarea::MatTextArea;

use std::rc::Rc;

type ValidityTransformFn = fn(String, NativeValidityState) -> ValidityState;

#[derive(Clone)]
pub struct ValidityTransform(
    pub Rc<ValidityTransformFn>
);
