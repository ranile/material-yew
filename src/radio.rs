use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener, read_boolean_property};
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Radio;

    #[wasm_bindgen(getter, static_method_of = Radio)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Radio);

pub struct MatRadio {
    link: ComponentLink<Self>,
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>
}

pub enum Msg {
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
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Radio::ensure_loaded();
        Self { props, link, node_ref: NodeRef::default(), closure: None }
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
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
        if self.props.checked {
            element.set_attribute("checked", &self.props.checked.to_string());
        } else {
            element.remove_attribute("checked");
        }

        if first_render {
            let callback = self.props.onchange.clone();
            add_event_listener(&self.node_ref, "change", move || {
                callback.emit(read_boolean_property(&element, "checked"));
            }, &mut self.closure)
        }
    }
}
