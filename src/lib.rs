mod button;
mod radio;
pub use button::{Button, ButtonVariants};
pub use radio::{Radio};

use wasm_bindgen::prelude::*;
use yew::AttrValue;

macro_rules! import_material_web_module {
    ($module:literal) => {{
        #[wasm_bindgen::prelude::wasm_bindgen(module = $module)]
        extern "C" {
            #[wasm_bindgen(getter)]
            fn __dummy_loader() -> wasm_bindgen::JsValue;
        }

        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        {
            LOADED.call_once(|| {
                __dummy_loader();
            });
        }
    }};
}

pub(crate) use import_material_web_module;

pub(crate) fn load_core() {
    import_material_web_module!("/md-web/core.js");
}

fn js_value_or_null<T>(v: Option<T>) -> JsValue
where
    JsValue: From<T>,
{
    match v {
        Some(v) => JsValue::from(v),
        None => JsValue::NULL,
    }
}

fn js_value_from_string_or_null(v: Option<&AttrValue>) -> JsValue {
    match v {
        Some(v) => JsValue::from_str(&*v),
        None => JsValue::NULL,
    }
}
