use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::to_option;

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
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/circular-progress-four-color#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub density: u32,
    #[prop_or_default]
    pub closed: bool
}

component!(MatCircularProgressFourColor, Props, |props: &Props| html! {
<mwc-circular-progress-four-color
    indeterminate?=to_option(props.indeterminate)
    progress=props.progress
    density=props.density
    closed?=to_option(props.closed)
></mwc-circular-progress-four-color>
}, CircularProgressFourColor, "circular-progress-four-color");

