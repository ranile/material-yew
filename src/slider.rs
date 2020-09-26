use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option};
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Slider;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Slider)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Slider);

pub struct MatSlider {
    props: Props,
    node_ref: NodeRef,
    input_closure: Option<Closure<dyn FnMut(JsValue)>>,
    change_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

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
    /// This is `JsValue` because `Slider` is undocumented
    /// See: https://github.com/material-components/material-components-web-components/issues/1848
    #[prop_or_default]
    pub oninput: Callback<JsValue>,
    /// This is `JsValue` because `Slider` is undocumented
    /// See: https://github.com/material-components/material-components-web-components/issues/1848
    #[prop_or_default]
    pub onchange: Callback<JsValue>,
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
            // TODO parse values
            let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
            let oninput = self.props.oninput.clone();
            self.input_closure = Some(Closure::wrap(Box::new(move |val| {
                oninput.emit(val)
            }) as Box<dyn FnMut(JsValue)>));
            element.add_event_listener_with_callback("input", self.input_closure.as_ref().unwrap().as_ref().unchecked_ref());

            let onchange = self.props.onchange.clone();
            self.change_closure = Some(Closure::wrap(Box::new(move |val| {
                onchange.emit(val)
            }) as Box<dyn FnMut(JsValue)>));
            element.add_event_listener_with_callback("change", self.change_closure.as_ref().unwrap().as_ref().unchecked_ref());
        }
    }
}

