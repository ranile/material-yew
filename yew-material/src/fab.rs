use crate::to_option;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-fab.js")]
extern "C" {
    #[derive(Debug)]
    type Fab;

    #[wasm_bindgen(getter, static_method_of = Fab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Fab);

/// Props for [`MatFab`]
///
/// [MWC Documentation for properties](https://github.com/material-components/material-components-web-components/tree/master/packages/fab#propertiesattributes)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub mini: bool,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    #[prop_or_default]
    pub extended: bool,
    #[prop_or_default]
    pub show_icon_at_end: bool,
    #[prop_or_default]
    pub children: Children,
}

component!(
    MatFab,
    Props,
    |props: &Props| {
        html! {
            <mwc-fab
                label=props.label
                icon=props.icon
                mini?=to_option(props.mini)
                reducedTouchTarget?=to_option(props.reduced_touch_target)
                extended?=to_option(props.extended)
                showIconAtEnd?=to_option(props.show_icon_at_end)
            >{ props.children.clone() }</mwc-fab>
        }
    },
    Fab,
    "fab"
);
