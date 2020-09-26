use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, read_boolean_property};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Switch;

    #[wasm_bindgen(getter, static_method_of = Switch)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Switch);

pub struct MatSwitch {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatSwitch {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Switch::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
              <mwc-switch
                  disabled=self.props.disabled
                  ref=self.node_ref.clone()
              ></mwc-switch>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
        if self.props.checked {
            element.set_attribute("checked", "checked");
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


