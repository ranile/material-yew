use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-icon-button.js")]
extern "C" {
    #[derive(Debug)]
    type IconButton;

    #[wasm_bindgen(getter, static_method_of = IconButton)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButton);

/// Props for [`MatIconButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct IconButtonProps {
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

component!(
    MatIconButton,
    IconButtonProps,
    |props: &IconButtonProps| {
        html! {
             <mwc-icon-button
                 label={props.label.clone()}
                 icon={props.icon.clone()}
                 disabled={props.disabled}
             >{props.children.clone()}</mwc-icon-button>
        }
    },
    IconButton,
    "icon-button"
);
