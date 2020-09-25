use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::list::{GraphicType, request_selected_listener};
use crate::to_option;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type CheckListItem;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CheckListItem)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CheckListItem);

pub struct MatCheckListItem {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub left: bool,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub disabled: bool,
    // This is currently a JsValue because I don't want to depend on serde to try and extract the data from it
    // It is up to ths user to do whatever they want from it
    // It's probably possible to never touch this event but I'm gonna keep it for the sake of completeness
    #[prop_or_default]
    pub on_request_selected: Callback<JsValue>,
    pub children: Children,
}

impl Component for MatCheckListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        CheckListItem::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-check-list-item
                left?=to_option(self.props.left)
                graphic=self.props.graphic.to_string()
                disabled=self.props.disabled
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-check-list-item>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            request_selected_listener(&self.node_ref, self.props.on_request_selected.clone(), &mut self.closure);
        }
    }
}
