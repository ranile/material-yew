use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::to_option;

#[wasm_bindgen(module = "/build/built-js.js")]
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
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/circular-progress#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub density: u32,
    #[prop_or_default]
    pub closed: bool,
}

component!(MatCircularProgress, Props, |props: &Props| html! {
    <mwc-circular-progress
        indeterminate?=to_option(props.indeterminate)
        progress=props.progress
        density=props.density
        closed?=to_option(props.closed)
    ></mwc-circular-progress>
}, CircularProgress, "circular-progress");
