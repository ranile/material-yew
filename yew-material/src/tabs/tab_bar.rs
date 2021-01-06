use crate::add_event_listener_with_one_param;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CustomEvent;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-tab-bar.js")]
extern "C" {
    #[derive(Debug)]
    type TabBar;

    #[wasm_bindgen(getter, static_method_of = TabBar)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TabBar);

/// The `mwc-tab-bar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar)
pub struct MatTabBar {
    props: TabBarProps,
    node_ref: NodeRef,
    activated_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for `MatTabBar`.
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/tab-bar#events)
#[derive(Debug, Properties, Clone)]
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TabBar::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            activated_closure: None,
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
            <mwc-tab-bar
                activeIndex=self.props.active_index
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-tab-bar>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let on_activated = self.props.onactivated.clone();
        if first_render {
            add_event_listener_with_one_param(
                &self.node_ref,
                "MDCTabBar:activated",
                move |value| {
                    let event = value.unchecked_into::<CustomEvent>();
                    let detail = event.detail().unchecked_into::<ActivatedDetailJS>();
                    on_activated.emit(detail.index());
                },
                &mut self.activated_closure,
            )
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
