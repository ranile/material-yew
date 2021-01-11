mod action_items;
mod navigation_icon;
mod title;

pub use action_items::*;
pub use navigation_icon::*;
pub use title::*;

use crate::to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-top-app-bar.js")]
extern "C" {
    #[derive(Debug)]
    type TopAppBar;

    #[wasm_bindgen(getter, static_method_of = TopAppBar)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TopAppBar);

/// The `mwc-top-app-bar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar)
pub struct MatTopAppBar {
    props: TopAppBarProps,
    node_ref: NodeRef,
    nav_listener: Option<EventListener>,
}

/// Props for [`MatTopAppBar`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#events)
#[derive(Debug, Properties, Clone)]
pub struct TopAppBarProps {
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

impl Component for MatTopAppBar {
    type Message = ();
    type Properties = TopAppBarProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TopAppBar::ensure_loaded();
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
            <mwc-top-app-bar
                centerTitle?=to_option(self.props.center_title)
                dense?=to_option(self.props.dense)
                prominent?=to_option(self.props.prominent)
                ref=self.node_ref.clone()
            >
                { self.props.children.clone() }
            </mwc-top-app-bar>
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
