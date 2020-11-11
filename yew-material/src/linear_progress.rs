use crate::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-linear-progress.js")]
extern "C" {
    #[derive(Debug)]
    type LinearProgress;

    #[wasm_bindgen(getter, static_method_of = LinearProgress)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(LinearProgress);

/// Props for [`MatLinearProgress`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/linear-progress#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct Props {
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
    Props,
    |props: &Props| {
        html! {
            <mwc-linear-progress
                indeterminate?=to_option(props.indeterminate)
                progress=props.progress
                buffer=props.buffer
                reverse?=to_option(props.reverse)
                closed?=to_option(props.closed)
            ></mwc-linear-progress>
        }
    },
    LinearProgress,
    "linear-progress"
);
