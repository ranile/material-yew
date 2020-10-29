use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-icon-button.js")]
extern "C" {
    #[derive(Debug)]
    type IconButton;

    #[wasm_bindgen(getter, static_method_of = IconButton)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButton);

/// Props for [`MatIconButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

component!(MatIconButton, Props, |props: &Props| html! {
    <mwc-icon-button
        label=props.label
        icon=props.icon
        disabled=props.disabled
    >{ props.children.clone() }</mwc-icon-button>
}, IconButton, "icon-button");
