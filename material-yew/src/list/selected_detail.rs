use crate::list::ListIndex;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// The `RequestSelectedDetail` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-2)
#[derive(Debug)]
pub struct SelectedDetail {
    pub index: ListIndex,
    pub diff: Option<IndexDiff>,
}

/// Type for [`SelectedDetail::diff`]
///
/// See `**` [here on MWC documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-2).
#[derive(Debug)]
pub struct IndexDiff {
    pub added: Vec<usize>,
    pub removed: Vec<usize>,
}

impl From<JsValue> for SelectedDetail {
    fn from(value: JsValue) -> Self {
        let detail = value.unchecked_into::<SelectedDetailJS>();
        let index = ListIndex::from(detail.index());

        let diff = if detail.diff().is_undefined() {
            None
        } else {
            let diff = detail.diff();
            Some(IndexDiff {
                added: diff.added(),
                removed: diff.removed(),
            })
        };
        Self { index, diff }
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type SelectedDetailJS;

    #[wasm_bindgen(method, getter)]
    pub fn index(this: &SelectedDetailJS) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn diff(this: &SelectedDetailJS) -> IndexDiffJS;

    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type IndexDiffJS;

    #[wasm_bindgen(method, getter)]
    pub fn added(this: &IndexDiffJS) -> Vec<usize>;

    #[wasm_bindgen(method, getter)]
    pub fn removed(this: &IndexDiffJS) -> Vec<usize>;
}
