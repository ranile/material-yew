use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::to_option;
use crate::list::request_selected::request_selected_listener;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type RadioListItem;

    #[wasm_bindgen(getter, static_method_of = RadioListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(RadioListItem);

pub struct MatRadioListItem {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub left: bool,
    #[prop_or_default]
    pub group: Option<String>,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for MatRadioListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        RadioListItem::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-radio-list-item
                left?=to_option(self.props.left)
                graphic=self.props.graphic.to_string()
                group=self.props.group.as_ref().unwrap_or(&"null".to_string())
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-radio-list-item>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            request_selected_listener(&self.node_ref, self.props.on_request_selected.clone(), &mut self.closure);
        }
    }
}
