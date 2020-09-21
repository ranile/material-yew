use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::to_option;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Formfield;

    #[wasm_bindgen(getter, static_method_of = Formfield)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Formfield);

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub align_end: bool,
    #[prop_or_default]
    pub space_between: bool,
    #[prop_or_default]
    pub nowrap: bool,
}

component!(MatFormfield, Props, |self_: &MatFormfield| html! {
    <mwc-formfield
        label=self_.props.label
        alignEnd?=to_option(self_.props.align_end)
        spaceBetween?=to_option(self_.props.space_between)
        nowrap?=to_option(self_.props.nowrap)
    >{ self_.props.children.clone() }</mwc-formfield>
}, Formfield);

