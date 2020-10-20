#[cfg(feature = "textfield")]
pub mod textfield;
#[cfg(feature = "textfield")]
pub use textfield::{MatTextField};

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod validity_state;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use validity_state::{ValidityState};

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod text_field_type;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use text_field_type::TextFieldType;

#[cfg(feature = "textarea")]
pub mod textarea;
#[cfg(feature = "textarea")]
pub use textarea::MatTextArea;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use web_sys::ValidityState as NativeValidityState;

use std::rc::Rc;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub type ValidityTransformFn = dyn Fn(String, NativeValidityState) -> ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
#[derive(Clone)]
pub struct ValidityTransform(
    pub Rc<ValidityTransformFn>
);

impl ValidityTransform {
    pub(crate) fn new<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(func: F) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}
