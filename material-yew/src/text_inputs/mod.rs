#[cfg(feature = "textfield")]
mod textfield;
#[cfg(feature = "textfield")]
pub use textfield::*;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) mod validity_state;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use validity_state::ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) mod text_field_type;
#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use text_field_type::*;

#[cfg(feature = "textarea")]
mod textarea;
#[cfg(feature = "textarea")]
pub use textarea::*;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub use web_sys::ValidityState as NativeValidityState;

use std::rc::Rc;

use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Event, InputEvent};
use yew::{Callback, NodeRef};

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub(crate) type ValidityTransformFn = dyn Fn(String, NativeValidityState) -> ValidityState;

#[cfg(any(feature = "textfield", feature = "textarea"))]
#[derive(Clone)]
/// Owned function for validity props
pub struct ValidityTransform(pub(crate) Rc<ValidityTransformFn>);

#[cfg(any(feature = "textfield", feature = "textarea"))]
impl ValidityTransform {
    pub(crate) fn new<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}

impl PartialEq for ValidityTransform {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

fn set_on_input_handler(
    node_ref: &NodeRef,
    callback: Callback<String>,
    convert: impl Fn((InputEvent, JsValue)) -> String + 'static,
) -> EventListener {
    let element = node_ref.cast::<Element>().unwrap();
    EventListener::new(&element, "input", move |event: &Event| {
        let js_value = JsValue::from(event);

        if let Some(input_event) = JsCast::dyn_ref::<web_sys::InputEvent>(&js_value) {
            callback.emit(convert((input_event.clone(), js_value)))
        } else {
            panic!("could not convert to `InputEvent`");
        }
    })
}
