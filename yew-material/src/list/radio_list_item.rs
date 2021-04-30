use crate::bool_to_option;
use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-radio-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type RadioListItem;

    #[wasm_bindgen(getter, static_method_of = RadioListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(RadioListItem);

/// The `mwc-list-item` Component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-radio-list-item)
pub struct MatRadioListItem {
    props: RadioListItemProps,
    node_ref: NodeRef,
    request_selected_listener: Option<EventListener>,
}

/// Props for [`MatRadioListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-radio-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-radio-list-item-2)
#[derive(Debug, Properties, Clone)]
pub struct RadioListItemProps {
    #[prop_or_default]
    pub left: bool,
    #[prop_or_default]
    pub group: Option<Cow<'static, str>>,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        RadioListItem::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            request_selected_listener: None,
        }
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
            <mwc-radio-list-item
                left=bool_to_option(self.props.left)
                graphic=self.props.graphic.to_string()
                group=self.props.group.as_ref().unwrap_or(&Cow::from("null"))
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-radio-list-item>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.request_selected_listener = Some(request_selected_listener(
                &self.node_ref,
                self.props.on_request_selected.clone(),
            ));
        }
    }
}
