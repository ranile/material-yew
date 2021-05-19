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
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct IconProps {
    pub children: Children,
}

component!(
    MatIcon,
    IconProps,
    |props: &IconProps| {
        html! {
            <mwc-icon>{ props.children.clone() }</mwc-icon>
        }
    },
    Icon,
    "icon"
);
