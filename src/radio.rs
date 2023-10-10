use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Whether or not the radio is disabled."]
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[doc = "The element value to use in form submission when checked."]
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or_default]
    pub checked: Option<bool>,
    #[doc = ""]
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[doc = ""]
    #[prop_or_default]
    pub form: Option<HTMLFormElement>,
    #[doc = ""]
    #[prop_or_default]
    pub labels: Option<NodeList>,
    #[prop_or_default]
    pub children: Html,
}

#[wasm_bindgen(module = "/md-web/radio.js")]
extern "C" {
    #[wasm_bindgen(getter)]
    fn __dummy_loader() -> wasm_bindgen::JsValue;
}

#[allow(dead_code)]
static LOADED: std::sync::Once = std::sync::Once::new();


#[function_component]
pub fn Radio(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/radio.js");
        crate::load_core();
    });
    html! { <md-radio
       disabled={props.disabled.unwrap_or_default()}
       value={props.value.clone().unwrap_or_default()}
       ~checked={crate::js_value_or_null(props.checked.clone())}
       ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
       ~form={crate::js_value_or_null(props.form.clone())}
       ~labels={crate::js_value_or_null(props.labels.clone())}
    /> }
}
