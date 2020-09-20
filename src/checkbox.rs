use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener};
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/build/checkbox.js")]
extern "C" {
    #[derive(Debug)]
    type Checkbox;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Checkbox)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Checkbox, value: &str);
}

// call the macro with the type
loader_hack!(Checkbox);

pub struct MatCheckbox {
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
    pub indeterminate: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatCheckbox {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Checkbox::ensure_loaded();
        Self { props, link, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {        false    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
              <mwc-checkbox
                  indeterminate?=to_option(self.props.indeterminate)
                  disabled=self.props.disabled
                  value=self.props.value
                  reducedTouchTarget?=to_option(self.props.reduced_touch_target)
                  ref=self.node_ref.clone()
              ></mwc-checkbox>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
        if self.props.checked {
            element.set_attribute("checked", &self.props.checked.to_string());
        }
        if first_render {
            let callback = self.props.onchange.clone();
            let ele = element.clone();
            add_event_listener(&self.node_ref, "change", move || {
                let checked = match ele.get_attribute("checked") {
                    Some(_) => true,
                    _ => false
                };

                callback.emit(checked);
            }, &mut self.closure)
        }
    }
}
