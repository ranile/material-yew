use crate::list::request_selected::request_selected_listener;
use crate::list::{GraphicType, RequestSelectedDetail};
use crate::{to_option, to_option_string};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-list-item.js")]
extern "C" {
    #[derive(Debug)]
    type ListItem;

    #[wasm_bindgen(getter, static_method_of = ListItem)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(ListItem);

/// The `mwc-list-item` Component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-item)
pub struct MatListItem {
    props: ListItemProps,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for [`MatListItem`]
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-item-1)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-item-2)
#[derive(Debug, Properties, Clone)]
pub struct ListItemProps {
    #[prop_or_default]
    pub value: String,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListItem::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            closure: None,
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
            request_selected_listener(
                &self.node_ref,
                self.props.on_request_selected.clone(),
                &mut self.closure,
            );
        }
    }
}
