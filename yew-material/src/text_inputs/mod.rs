#[cfg(feature = "textfield")]
pub mod textfield;
#[cfg(feature = "textfield")]
pub use textfield::MatTextField;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod validity_state;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use validity_state::ValidityState;

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

use crate::add_event_listener_with_one_param;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use yew::{Callback, InputData, NodeRef};

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub type ValidityTransformFn = dyn Fn(String, NativeValidityState) -> ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
#[derive(Clone)]
pub struct ValidityTransform(pub Rc<ValidityTransformFn>);

#[cfg(any(feature = "textfield", feature = "textarea"))]
impl ValidityTransform {
    pub(crate) fn new<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}

fn set_on_input_handler(
    node_ref: &NodeRef,
    callback: Callback<InputData>,
    convert: impl Fn(JsValue) -> InputData + 'static,
    closure_to_store_in: &mut Option<Closure<dyn FnMut(JsValue)>>,
) {
    add_event_listener_with_one_param(
        node_ref,
        "input",
        move |value| callback.emit(convert(value)),
        closure_to_store_in,
    );
}
