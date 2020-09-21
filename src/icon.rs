use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Icon;

    #[wasm_bindgen(getter, static_method_of = Icon)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Icon);

#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub children: Children,
}

component!(MatIcon, Props, |props: &Props| html! {
    <mwc-icon>{ props.children.clone() }</mwc-icon>
}, Icon);
