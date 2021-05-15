use crate::{bool_to_option, event_details_into};
use gloo::events::EventListener;
use js_sys::Object;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/tab)
pub struct MatTab {
    props: TabProps,
    node_ref: NodeRef,
    interacted_listener: Option<EventListener>,
}

/// Props for `MatTab`
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/tab#events)
#[derive(Debug, Properties, Clone)]
pub struct TabProps {
    #[prop_or_default]
    pub label: Cow<'static, str>,
    #[prop_or_default]
    pub icon: Cow<'static, str>,
    #[prop_or_default]
    pub has_image_icon: bool,
    #[prop_or_default]
    pub indicator_icon: Cow<'static, str>,
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
            interacted_listener: None,
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
                label=self.props.label.clone()
                icon=self.props.icon.clone()
                hasImageIcon=bool_to_option(self.props.has_image_icon)
                indicatorIcon=self.props.indicator_icon.clone()
                isFadingIndicator=bool_to_option(self.props.is_fading_indicator)
                minWidth=bool_to_option(self.props.min_width)
                isMinWidthIndicator=bool_to_option(self.props.is_min_width_indicator)
                stacked=bool_to_option(self.props.stacked)
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-tab>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.node_ref.cast::<Element>().unwrap();

            let on_interacted = self.props.oninteracted.clone();
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
