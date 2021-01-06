use crate::{add_event_listener_with_one_param, to_option};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-tab.js")]
extern "C" {
    #[derive(Debug)]
    type Tab;

    #[wasm_bindgen(getter, static_method_of = Tab)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Tab);

/// The `mwc-tab` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/tab)
pub struct MatTab {
    props: TabProps,
    node_ref: NodeRef,
    interacted_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for `MatTab`
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#events)
#[derive(Debug, Properties, Clone)]
pub struct TabProps {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub has_image_icon: bool,
    #[prop_or_default]
    pub indicator_icon: String,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Tab::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            interacted_closure: None,
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
            <mwc-tab
                label=self.props.label
                icon=self.props.icon
                hasImageIcon?=to_option(self.props.has_image_icon)
                indicatorIcon=self.props.indicator_icon
                isFadingIndicator?=to_option(self.props.is_fading_indicator)
                minWidth?=to_option(self.props.min_width)
                isMinWidthIndicator?=to_option(self.props.is_min_width_indicator)
                stacked?=to_option(self.props.stacked)
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-tab>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let on_interacted = self.props.oninteracted.clone();
        if first_render {
            add_event_listener_with_one_param(
                &self.node_ref,
                "MDCTab:interacted",
                move |value| {
                    let event = value.unchecked_into::<CustomEvent>();
                    let detail = event.detail().unchecked_into::<InteractedDetailJS>();
                    on_interacted.emit(detail.tab_id());
                },
                &mut self.interacted_closure,
            )
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
