use crate::{bool_to_option, to_option_string};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-circular-progress-four-color.js")]
extern "C" {
    #[derive(Debug)]
    type CircularProgressFourColor;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CircularProgressFourColor)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CircularProgressFourColor);

/// Props for [`MatCircularProgressFourColor`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/circular-progress-four-color#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CircularProgressFourColorProps {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub density: u32,
    #[prop_or_default]
    pub closed: bool,
}

component!(
    MatCircularProgressFourColor,
    CircularProgressFourColorProps,
    |props: &CircularProgressFourColorProps| {
        html! {
         <mwc-circular-progress-four-color
             indeterminate={bool_to_option(props.indeterminate)}
             progress={to_option_string(props.progress)}
             density={to_option_string(props.density)}
             closed={bool_to_option(props.closed)}
         ></mwc-circular-progress-four-color>
        }
    },
    CircularProgressFourColor,
    "circular-progress-four-color"
);
