use std::collections::HashSet;
use wasm_bindgen::{JsValue, JsCast};

/// The `MWCListIndex` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-1)
#[derive(Debug)]
pub enum ListIndex {
    /// Provided when `multi` prop is set to `true` on the component
    ///
    /// `None` denotes value os `-1`
    Single(Option<usize>),
    /// Provided when `multi` prop is set to `false` (default) on the component
    Multi(HashSet<usize>),
}


impl From<JsValue> for ListIndex {
    fn from(val: JsValue) -> Self {
        if let Ok(set) = val.clone().dyn_into::<js_sys::Set>() {
            let indices = set.values()
                .into_iter()
                .filter_map(|item| item.ok())
                .filter_map(|value| value.as_f64())
                .map(|num| num as usize)
                .collect();
            ListIndex::Multi(indices)
        } else if let Some(value) = val.as_f64() {
            ListIndex::Single(if value != -1.0 { Some(value as usize) } else { None })
        } else {
            panic!("This should never happen")
        }
    }
}

