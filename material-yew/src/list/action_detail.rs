use crate::list::ListIndex;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// The `ActionDetail` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-2)
#[derive(Debug)]
pub struct ActionDetail {
    #[allow(dead_code)]
    index: ListIndex,
}

impl From<JsValue> for ActionDetail {
    fn from(value: JsValue) -> Self {
        let detail = value.unchecked_into::<ActionDetailJs>();
        let index = ListIndex::from(detail.index());
        Self { index }
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type ActionDetailJs;

    #[wasm_bindgen(method, getter)]
    pub fn index(this: &ActionDetailJs) -> JsValue;
}
