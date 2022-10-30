use crate::{event_details_into, to_option_string};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-tab-bar.js")]
extern "C" {
    #[derive(Debug)]
    type TabBar;

    #[wasm_bindgen(getter, static_method_of = TabBar)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TabBar);

/// The `mwc-tab-bar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab-bar)
pub struct MatTabBar {
    node_ref: NodeRef,
    activated_listener: Option<EventListener>,
}

/// Props for `MatTabBar`.
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab-bar#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/tab-bar#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TabBarProps {
    #[prop_or_default]
    pub active_index: u32,
    /// Binds to `MDCTabBar:activated` event on `mwc-tab`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onactivated: Callback<usize>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatTabBar {
    type Message = ();
    type Properties = TabBarProps;

    fn create(_: &Context<Self>) -> Self {
        TabBar::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            activated_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-tab-bar
                 activeIndex={to_option_string(props.active_index)}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-tab-bar>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        if self.activated_listener.is_none() {
            let element = self.node_ref.cast::<Element>().unwrap();

            let on_activated = props.onactivated.clone();
            self.activated_listener = Some(EventListener::new(
                &element,
                "MDCTabBar:activated",
                move |event| {
                    let detail = event_details_into::<ActivatedDetailJS>(event);
                    on_activated.emit(detail.index());
                },
            ));
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type ActivatedDetailJS;

    #[wasm_bindgen(method, getter)]
    fn index(this: &ActivatedDetailJS) -> usize;
}
