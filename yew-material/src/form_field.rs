use crate::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-formfield.js")]
extern "C" {
    #[derive(Debug)]
    type Formfield;

    #[wasm_bindgen(getter, static_method_of = Formfield)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Formfield);

/// Props for [`MatFormfield`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/formfield#propertiesattributes)
#[derive(Properties, Clone)]
pub struct FormfieldProps {
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

component!(
    MatFormfield,
    FormfieldProps,
    |props: &FormfieldProps| {
        html! {
            <mwc-formfield
                label=props.label
                alignEnd?=to_option(props.align_end)
                spaceBetween?=to_option(props.space_between)
                nowrap?=to_option(props.nowrap)
            >{ props.children.clone() }</mwc-formfield>
        }
    },
    Formfield,
    "formfield"
);
