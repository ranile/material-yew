use crate::{bool_to_option, to_option_string};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, Element};
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-slider.js")]
extern "C" {
    #[derive(Debug)]
    type Slider;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Slider)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Slider);

/// The `mwc-snackbar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/slider)
pub struct MatSlider {
    node_ref: NodeRef,
    input_listener: Option<EventListener>,
    change_listener: Option<EventListener>,
}

/// Props for [`MatSlider`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/slider#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/slider#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct SliderProps {
    #[prop_or(0)]
    pub value: u32,
    #[prop_or(0)]
    pub min: u32,
    #[prop_or(100)]
    pub max: u32,
    #[prop_or(1)]
    pub step: u32,
    #[prop_or(false)]
    pub pin: bool,
    #[prop_or(false)]
    pub markers: bool,
    /// Binds to input on `mwc-slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is
    /// undocumented See: <https://github.com/material-components/material-components-web-components/issues/1848>
    #[prop_or_default]
    pub oninput: Callback<CustomEvent>,
    /// Binds to change on `mwc-slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is
    /// undocumented See: <https://github.com/material-components/material-components-web-components/issues/1848>
    #[prop_or_default]
    pub onchange: Callback<CustomEvent>,
}

impl Component for MatSlider {
    type Message = ();
    type Properties = SliderProps;

    fn create(_: &Context<Self>) -> Self {
        Slider::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            input_listener: None,
            change_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-slider
                 value={to_option_string(props.value)}
                 min={to_option_string(props.min)}
                 max={to_option_string(props.max)}
                 step={to_option_string(props.step)}
                 pin={bool_to_option(props.pin)}
                 markers={bool_to_option(props.markers)}
                 ref={self.node_ref.clone()}
             ></mwc-slider>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Element>().unwrap();
        if self.input_listener.is_none() {
            let oninput = props.oninput.clone();
            self.input_listener = Some(EventListener::new(&element, "input", move |event| {
                oninput.emit(JsValue::from(event).unchecked_into::<CustomEvent>())
            }));
        };

        if self.change_listener.is_none() {
            let onchange = props.onchange.clone();
            self.change_listener = Some(EventListener::new(&element, "change", move |event| {
                onchange.emit(JsValue::from(event).unchecked_into::<CustomEvent>())
            }));
        }
    }
}
