use crate::bool_to_option;
use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-radio-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type RadioListItem;

    #[wasm_bindgen(getter, static_method_of = RadioListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(RadioListItem);

/// The `mwc-list-item` Component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-radio-list-item)
pub struct MatRadioListItem {
    node_ref: NodeRef,
    request_selected_listener: Option<EventListener>,
}

/// Props for [`MatRadioListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-radio-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-radio-list-item-2)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct RadioListItemProps {
    #[prop_or_default]
    pub left: bool,
    #[prop_or_default]
    pub group: Option<AttrValue>,
    #[prop_or(GraphicType::Control)]
    pub graphic: GraphicType,
    /// Binds to `request-selected` event on `mwc-list-item`.
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for MatRadioListItem {
    type Message = ();
    type Properties = RadioListItemProps;

    fn create(_: &Context<Self>) -> Self {
        RadioListItem::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            request_selected_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-radio-list-item
                 left={bool_to_option(props.left)}
                 graphic={props.graphic.to_string()}
                 group={props.group.clone().unwrap_or_else(|| AttrValue::from("null"))}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-radio-list-item>
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
