use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::{bool_to_option, to_option_string};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type ListItem;

    #[wasm_bindgen(getter, static_method_of = ListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(ListItem);

/// The `mwc-list-item` Component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item)
pub struct MatListItem {
    node_ref: NodeRef,
    request_selected_listener: Option<EventListener>,
}

/// Props for [`MatListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-2)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ListItemProps {
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub group: bool,
    #[prop_or(- 1)]
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
    /// Binds to `request-selected` event on `mwc-list-item`.
    #[prop_or_default]
    pub on_request_selected: Callback<RequestSelectedDetail>,
    pub children: Children,
}

impl Component for MatListItem {
    type Message = ();
    type Properties = ListItemProps;

    fn create(_: &Context<Self>) -> Self {
        ListItem::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            request_selected_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-list-item
                 value={props.value.clone()}
                 group={bool_to_option(props.group)}
                 tabindex={to_option_string(props.tabindex)}
                 disabled={props.disabled}
                 twoline={bool_to_option(props.twoline)}
                 activated={bool_to_option(props.activated)}
                 graphic={to_option_string(props.graphic.to_string())}
                 multipleGraphics={bool_to_option(props.multiple_graphics)}
                 hasMeta={bool_to_option(props.has_meta)}
                 noninteractive={bool_to_option(props.noninteractive)}
                 selected={props.selected}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-list-item>
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
