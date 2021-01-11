use crate::to_option;
#[doc(inline)]
pub use crate::top_app_bar::{
    MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle,
};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-top-app-bar-fixed.js")]
extern "C" {
    #[derive(Debug)]
    type TopAppBarFixed;

    #[wasm_bindgen(getter, static_method_of = TopAppBarFixed)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TopAppBarFixed);

/// The `mwc-top-app-bar-fixed` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed)
pub struct MatTopAppBarFixed {
    props: TopAppBarFixedProps,
    node_ref: NodeRef,
    nav_listener: Option<EventListener>,
}

/// Props for [`MatTopAppBarFixed`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar-fixed#events)
#[derive(Debug, Properties, Clone)]
pub struct TopAppBarFixedProps {
    pub children: Children,
    #[prop_or_default]
    pub center_title: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub prominent: bool,
    /// Binds to `MDCTopAppBar:nav`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onnavigationiconclick: Callback<()>,
}

impl Component for MatTopAppBarFixed {
    type Message = ();
    type Properties = TopAppBarFixedProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TopAppBarFixed::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            nav_listener: None,
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
            <mwc-top-app-bar-fixed
                centerTitle?=to_option(self.props.center_title)
                dense?=to_option(self.props.dense)
                prominent?=to_option(self.props.prominent)
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-top-app-bar-fixed>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.props.onnavigationiconclick.clone();
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
