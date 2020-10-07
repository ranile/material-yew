use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener};
use web_sys::Node;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Radio;

    #[wasm_bindgen(getter, static_method_of = Radio)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Radio) -> bool;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Radio, value: bool);
}

loader_hack!(Radio);

pub struct MatRadio {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>
}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub global: bool,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatRadio {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Radio::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {        false    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
              <mwc-radio
                  disabled=self.props.disabled
                  name=self.props.name
                  value=self.props.value
                  global?=to_option(self.props.global)
                  reducedTouchTarget?=to_option(self.props.reduced_touch_target)
                  ref=self.node_ref.clone()
              ></mwc-radio>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Radio>().unwrap();
        element.set_checked(self.props.checked);

        if first_render {
            let callback = self.props.onchange.clone();
            add_event_listener(&self.node_ref, "change", move || {
                callback.emit(element.checked());
            }, &mut self.closure)
        }
    }
}
