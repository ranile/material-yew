use std::collections::HashSet;
use wasm_bindgen::{JsValue, JsCast};

#[derive(Debug)]
pub enum ListIndex {
    Single(Option<usize>),
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

