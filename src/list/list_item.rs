use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::{to_option, to_option_string};
use crate::list::request_selected::request_selected_listener;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type ListItem;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = ListItem)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(ListItem);

pub struct MatListItem {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub group: bool,
    #[prop_or(-1)]
    pub tabindex: i32,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub twoline: bool,
    #[prop_or_default]
    pub activated: bool,
    #[prop_or(GraphicType::Null)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub multiple_graphics: bool,
    #[prop_or_default]
    pub has_meta: bool,
    #[prop_or_default]
    pub noninteractive: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for MatListItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListItem::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-list-item
                value?=to_option_string(&self.props.value)
                group?=to_option(self.props.group)
                tabindex=self.props.tabindex
                disabled=self.props.disabled
                twoline?=to_option(self.props.twoline)
                activated?=to_option(self.props.activated)
                graphic=self.props.graphic.to_string()
                multipleGraphics?=to_option(self.props.multiple_graphics)
                hasMeta?=to_option(self.props.has_meta)
                noninteractive?=to_option(self.props.noninteractive)
                selected=self.props.selected
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-list-item>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            request_selected_listener(&self.node_ref, self.props.on_request_selected.clone(), &mut self.closure);
        }
    }
}

