use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::to_option;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Button;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Button)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Button);

#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub raised: bool,
    #[prop_or_default]
    pub unelevated: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub trailing_icon: bool,
}

component!(MatButton, Props, |props: &Props| html! {
    <mwc-button
        label=props.label
        icon?=props.icon.as_ref()
        raised?=to_option(props.raised)
        unelevated?=to_option(props.unelevated)
        outlined?=to_option(props.outlined)
        dense?=to_option(props.dense)
        trailingIcon?=to_option(props.trailing_icon)
        disabled=props.disabled
    ></mwc-button>
}, Button);
