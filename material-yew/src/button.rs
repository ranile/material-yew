use crate::bool_to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-button.js")]
extern "C" {
    #[derive(Debug)]
    type Button;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Button)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Button);

/// Props for [`MatButton`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/button#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
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

component!(
    MatButton,
    ButtonProps,
    |props: &ButtonProps| {
        html! {
             <mwc-button
             icon={props.icon.clone()}
             label={props.label.clone()}
             disabled={props.disabled}
             raised={bool_to_option(props.raised)}
             unelevated={bool_to_option(props.unelevated)}
             outlined={bool_to_option(props.outlined)}
             dense={bool_to_option(props.dense)}
             trailingIcon={bool_to_option(props.trailing_icon)}
             ></mwc-button>
        }
    },
    Button,
    "button"
);
