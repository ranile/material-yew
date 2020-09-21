use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::to_option;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Fab;

    #[wasm_bindgen(getter, static_method_of = Fab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Fab);

pub struct MatFab {
    props: Props
}

pub enum Msg {}

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

impl Component for MatFab {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Fab::ensure_loaded();
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-fab
                label=self.props.label
                icon=self.props.icon
                mini?=to_option(self.props.mini)
                reducedTouchTarget?=to_option(self.props.reduced_touch_target)
                extended?=to_option(self.props.extended)
                showIconAtEnd?=to_option(self.props.show_icon_at_end)
            >{ self.props.children.clone() }</mwc-fab>
        }
    }
}
