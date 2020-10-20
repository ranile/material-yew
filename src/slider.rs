use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener_with_one_param};
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/slider)
pub struct MatSlider {
    props: Props,
    node_ref: NodeRef,
    input_closure: Option<Closure<dyn FnMut(JsValue)>>,
    change_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for [`MatSlider`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/slider#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/slider#events)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or(0)]
    pub value: u32,
    #[prop_or(0)]
    pub min: u32,
    #[prop_or(100)]
    pub max: u32,
    #[prop_or(0)]
    pub step: u32,
    #[prop_or(false)]
    pub pin: bool,
    #[prop_or(false)]
    pub markers: bool,
    /// Binds to input on `mwc-slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is undocumented
    /// See: https://github.com/material-components/material-components-web-components/issues/1848
    #[prop_or_default]
    pub oninput: Callback<CustomEvent>,
    /// Binds to change on `mwc-slider`
    /// Type passed to callback is `CustomEvent` because `Slider` is undocumented
    /// See: https://github.com/material-components/material-components-web-components/issues/1848
    #[prop_or_default]
    pub onchange: Callback<CustomEvent>,
}

impl Component for MatSlider {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Slider::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), input_closure: None, change_closure: None, }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-slider
                value=self.props.value
                min=self.props.min
                max=self.props.max
                step=self.props.step
                pin?=to_option(self.props.pin)
                markers?=to_option(self.props.markers)
                ref=self.node_ref.clone()
            ></mwc-slider>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let oninput = self.props.oninput.clone();
            add_event_listener_with_one_param(&self.node_ref, "input", move |val| {
                oninput.emit(val.unchecked_into::<CustomEvent>())
            }, &mut self.input_closure);

            let onchange = self.props.onchange.clone();
            add_event_listener_with_one_param(&self.node_ref, "change", move |val| {
                onchange.emit(val.unchecked_into::<CustomEvent>())
            }, &mut self.change_closure);
        }
    }
}

