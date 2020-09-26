use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, read_boolean_property, to_option};
use wasm_bindgen::JsCast;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type IconButtonToggle;

    #[wasm_bindgen(getter, static_method_of = IconButtonToggle)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(IconButtonToggle);

pub struct MatIconButtonToggle {
    link: ComponentLink<Self>,
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub on: bool,
    #[prop_or_default]
    pub on_icon: String,
    #[prop_or_default]
    pub off_icon: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatIconButtonToggle {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        IconButtonToggle::ensure_loaded();
        Self { props, link, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {

        html! {
            <mwc-icon-button-toggle
                on?=to_option(self.props.on)
                onIcon=self.props.on_icon
                offIcon=self.props.off_icon
                label=self.props.label
                disabled=self.props.disabled
                ref=self.node_ref.clone()
            > { self.props.children.clone() }</mwc-icon-button-toggle>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();

        if first_render {
            let callback = self.props.onchange.clone();
            let el = element.clone();
            add_event_listener(&self.node_ref, "MDCIconButtonToggle:change",move || {
                let is_on = read_boolean_property(&el, "on");
                callback.emit(is_on)
            }, &mut self.closure);
        }
    }
}
