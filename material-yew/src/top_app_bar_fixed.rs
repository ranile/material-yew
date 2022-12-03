use crate::bool_to_option;
#[doc(inline)]
pub use crate::top_app_bar::{
    MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle,
};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-top-app-bar-fixed.js")]
extern "C" {
    #[derive(Debug)]
    type TopAppBarFixed;

    #[wasm_bindgen(getter, static_method_of = TopAppBarFixed)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TopAppBarFixed);

/// The `mwc-top-app-bar-fixed` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/top-app-bar-fixed)
pub struct MatTopAppBarFixed {
    node_ref: NodeRef,
    nav_listener: Option<EventListener>,
}

/// Props for [`MatTopAppBarFixed`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/top-app-bar-fixed#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/top-app-bar-fixed#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct TopAppBarFixedProps {
    pub children: Children,
    #[prop_or_default]
    pub center_title: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub prominent: bool,
    #[prop_or_default]
    pub short: bool,
    /// Binds to `MDCTopAppBar:nav`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onnavigationiconclick: Callback<()>,
}

impl Component for MatTopAppBarFixed {
    type Message = ();
    type Properties = TopAppBarFixedProps;

    fn create(_: &Context<Self>) -> Self {
        TopAppBarFixed::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            nav_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-top-app-bar-fixed
                 centerTitle={bool_to_option(props.center_title)}
                 dense={bool_to_option(props.dense)}
                 prominent={bool_to_option(props.prominent)}
                 short={bool_to_option(props.short)}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-top-app-bar-fixed>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clear nav_listener in case a new callback was registered
        self.nav_listener = None;
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        if self.nav_listener.is_none() {
            let callback = props.onnavigationiconclick.clone();
            let element = self.node_ref.cast::<Element>().unwrap();

            self.nav_listener = Some(EventListener::new(
                &element,
                "MDCTopAppBar:nav",
                move |_| {
                    callback.emit(());
                },
            ));
        }
    }
}
