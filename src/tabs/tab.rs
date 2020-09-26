use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Tab;

    #[wasm_bindgen(getter, static_method_of = Tab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Tab);

pub struct MatTab {
    props: Props,
    node_ref: NodeRef,
    // TODO
    interacted_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub has_image_icon: bool,
    #[prop_or_default]
    pub indicator_icon: String,
    #[prop_or_default]
    pub is_fading_indicator: bool,
    #[prop_or_default]
    pub min_width: bool,
    #[prop_or_default]
    pub is_min_width_indicator: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub oninteracted: Callback<JsValue>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatTab {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tab::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), interacted_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-tab
                label=self.props.label
                icon=self.props.icon
                hasImageIcon?=to_option(self.props.has_image_icon)
                indicatorIcon=self.props.indicator_icon
                isFadingIndicator?=to_option(self.props.is_fading_indicator)
                minWidth?=to_option(self.props.min_width)
                isMinWidthIndicator?=to_option(self.props.is_min_width_indicator)
                stacked?=to_option(self.props.stacked)
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-tab>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {

        }
    }
}
