use crate::{bool_to_option, to_option_string};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-circular-progress.js")]
extern "C" {
    #[derive(Debug)]
    type CircularProgress;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CircularProgress)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CircularProgress);

/// Props for [`MatCircularProgress`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/circular-progress#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CircularProgressProps {
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
    MatCircularProgress,
    CircularProgressProps,
    |props: &CircularProgressProps| {
        html! {
             <mwc-circular-progress
                 indeterminate={bool_to_option(props.indeterminate)}
                 progress={to_option_string(props.progress)}
                 density={to_option_string(props.density)}
                 closed={bool_to_option(props.closed)}
             ></mwc-circular-progress>
        }
    },
    CircularProgress,
    "circular-progress"
);
