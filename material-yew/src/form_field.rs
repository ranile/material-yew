use crate::bool_to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-formfield.js")]
extern "C" {
    #[derive(Debug)]
    type Formfield;

    #[wasm_bindgen(getter, static_method_of = Formfield)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Formfield);

/// Props for [`MatFormfield`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/formfield#propertiesattributes)
#[derive(Properties, PartialEq, Clone)]
pub struct FormfieldProps {
    pub children: Children,
    #[prop_or_default]
    pub label: Option<AttrValue>,
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
                 label={props.label.clone()}
                 alignEnd={bool_to_option(props.align_end)}
                 spaceBetween={bool_to_option(props.space_between)}
                 nowrap={bool_to_option(props.nowrap)}
             >{props.children.clone()}</mwc-formfield>
        }
    },
    Formfield,
    "formfield"
);
