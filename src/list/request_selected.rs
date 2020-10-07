use yew::prelude::*;
use wasm_bindgen::prelude::*;
use js_sys::Object;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use crate::add_event_listener_with_one_param;

pub enum RequestSelectedSource {
    Interaction,
    Property,
}

pub struct RequestSelectedDetail {
    pub selected: bool,
    pub source: RequestSelectedSource,
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type RequestSelectedDetailJS;

    #[wasm_bindgen(method, getter)]
    fn selected(this: &RequestSelectedDetailJS) -> bool;

    #[wasm_bindgen(method, getter)]
    fn source(this: &RequestSelectedDetailJS) -> String;
}

pub fn request_selected_listener(node_ref: &NodeRef, callback: Callback<RequestSelectedDetail>, closure_to_store_in: &mut Option<Closure<dyn FnMut(JsValue)>>) {
    add_event_listener_with_one_param(node_ref, "request-selected", move |val: JsValue| {
        let event = val.unchecked_into::<CustomEvent>();
        let selected_detail = event.detail().unchecked_into::<RequestSelectedDetailJS>();
        let selected_detail = RequestSelectedDetail {
            selected: selected_detail.selected(),
            source: match selected_detail.source().as_str() {
                "interaction" => RequestSelectedSource::Interaction,
                "property" => RequestSelectedSource::Property,
                val => panic!(format!("Invalid `source` value {} received. This should never happen", val))
            }
        };
        callback.emit(selected_detail);
    }, closure_to_store_in)
}
