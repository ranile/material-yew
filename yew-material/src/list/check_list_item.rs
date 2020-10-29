use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::to_option;
use crate::list::request_selected::request_selected_listener;

#[wasm_bindgen(module = "/../build/mwc-check-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type CheckListItem;

    #[wasm_bindgen(getter, static_method_of = CheckListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(CheckListItem);

/// The `mwc-check-list-item` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#checklist)
pub struct MatCheckListItem {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for [`MatCheckListItem`]
///
/// MWC Documentation for [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-check-list-item)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-check-list-item-1)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub left: bool,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for MatCheckListItem {
    type Message = ();
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
