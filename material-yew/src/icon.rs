use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-icon.js")]
extern "C" {
    #[derive(Debug)]
    type Icon;

    #[wasm_bindgen(getter, static_method_of = Icon)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Icon);

/// Props for [`MatIcon`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct IconProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

component!(
    MatIcon,
    IconProps,
    |props: &IconProps| {
        html! {
             <mwc-icon class={props.class.to_owned()}>{props.children.clone()}</mwc-icon>
        }
    },
    Icon,
    "icon"
);
