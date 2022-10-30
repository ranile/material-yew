use crate::{bool_to_option, event_details_into};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-tab.js")]
extern "C" {
    #[derive(Debug)]
    type Tab;

    #[wasm_bindgen(getter, static_method_of = Tab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Tab);

/// The `mwc-tab` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab)
pub struct MatTab {
    node_ref: NodeRef,
    interacted_listener: Option<EventListener>,
}

/// Props for `MatTab`
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TabProps {
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    #[prop_or_default]
    pub has_image_icon: bool,
    #[prop_or_default]
    pub indicator_icon: Option<AttrValue>,
    #[prop_or_default]
    pub is_fading_indicator: bool,
    #[prop_or_default]
    pub min_width: bool,
    #[prop_or_default]
    pub is_min_width_indicator: bool,
    #[prop_or_default]
    pub stacked: bool,
    /// Binds to `MDCTab:interacted` event on `mwc-tab`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub oninteracted: Callback<String>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatTab {
    type Message = ();
    type Properties = TabProps;

    fn create(_: &Context<Self>) -> Self {
        Tab::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            interacted_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-tab
                 label={props.label.clone()}
                 icon={props.icon.clone()}
                 hasImageIcon={bool_to_option(props.has_image_icon)}
                 indicatorIcon={props.indicator_icon.clone()}
                 isFadingIndicator={bool_to_option(props.is_fading_indicator)}
                 minWidth={bool_to_option(props.min_width)}
                 isMinWidthIndicator={bool_to_option(props.is_min_width_indicator)}
                 stacked={bool_to_option(props.stacked)}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-tab>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        if self.interacted_listener.is_none() {
            let element = self.node_ref.cast::<Element>().unwrap();

            let on_interacted = props.oninteracted.clone();
            self.interacted_listener = Some(EventListener::new(
                &element,
                "MDCTab:interacted",
                move |event| {
                    let detail = event_details_into::<InteractedDetailJS>(event);
                    on_interacted.emit(detail.tab_id());
                },
            ));
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type InteractedDetailJS;

    #[wasm_bindgen(method, getter, js_name=tabId)]
    fn tab_id(this: &InteractedDetailJS) -> String;
}
