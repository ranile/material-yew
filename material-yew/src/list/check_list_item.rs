use crate::bool_to_option;
use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-check-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type CheckListItem;

    #[wasm_bindgen(getter, static_method_of = CheckListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(CheckListItem);

/// The `mwc-check-list-item` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#checklist)
pub struct MatCheckListItem {
    node_ref: NodeRef,
    request_selected_listener: Option<EventListener>,
}

/// Props for [`MatCheckListItem`]
///
/// MWC Documentation for [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-check-list-item)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-check-list-item-1)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CheckListItemProps {
    #[prop_or_default]
    pub left: bool,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    #[prop_or_default]
    pub selected: bool,
    pub children: Children,
}

impl Component for MatCheckListItem {
    type Message = ();
    type Properties = CheckListItemProps;

    fn create(_: &Context<Self>) -> Self {
        CheckListItem::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            request_selected_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-check-list-item
                 left={bool_to_option(props.left)}
                 graphic={props.graphic.as_str()}
                 disabled={props.disabled}
                 selected={props.selected}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-check-list-item>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        if self.request_selected_listener.is_none() {
            self.request_selected_listener = Some(request_selected_listener(
                &self.node_ref,
                props.on_request_selected.clone(),
            ));
        }
    }
}
