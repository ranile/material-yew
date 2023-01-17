use crate::{bool_to_option, to_option_string};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-linear-progress.js")]
extern "C" {
    #[derive(Debug)]
    type LinearProgress;

    #[wasm_bindgen(getter, static_method_of = LinearProgress)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(LinearProgress);

/// Props for [`MatLinearProgress`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/linear-progress#propertiesattributes)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct LinearProgressProps {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub buffer: f32,
    #[prop_or_default]
    pub reverse: bool,
    #[prop_or_default]
    pub closed: bool,
}

component!(
    MatLinearProgress,
    LinearProgressProps,
    |props: &LinearProgressProps| {
        html! {
             <mwc-linear-progress
                 indeterminate={bool_to_option(props.indeterminate)}
                 progress={to_option_string(props.progress)}
                 buffer={to_option_string(props.buffer)}
                 reverse={bool_to_option(props.reverse)}
                 closed={bool_to_option(props.closed)}
             ></mwc-linear-progress>
        }
    },
    LinearProgress,
    "linear-progress"
);
