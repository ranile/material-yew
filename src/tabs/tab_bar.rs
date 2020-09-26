use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, to_option};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type TabBar;

    #[wasm_bindgen(getter, static_method_of = TabBar)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TabBar);

pub struct MatTabBar {
    props: Props,
    node_ref: NodeRef,
    activated_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub active_index: u32,
    #[prop_or_default]
    pub onactivated: Callback<JsValue>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatTabBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabBar::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), activated_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-tab-bar
                activeIndex=self.props.active_index
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-tab-bar>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {

        }
    }
}
