mod off_icon;
mod on_icon;

pub use off_icon::*;
pub use on_icon::*;

use crate::bool_to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-icon-button-toggle.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type IconButtonToggle;

    #[wasm_bindgen(getter, static_method_of = IconButtonToggle)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn on(this: &IconButtonToggle) -> bool;
}

loader_hack!(IconButtonToggle);

/// The `mwc-icon-button-toggle` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button-toggle)
pub struct MatIconButtonToggle {
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatIconButtonToggle`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button-toggle#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/icon-button-toggle#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct IconButtonToggleProps {
    #[prop_or_default]
    pub on: bool,
    #[prop_or_default]
    pub on_icon: Option<AttrValue>,
    #[prop_or_default]
    pub off_icon: Option<AttrValue>,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    /// Binds to `MDCIconButtonToggle:change`.
    ///
    /// Callback's parameter is the `isOn` value passed
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatIconButtonToggle {
    type Message = ();
    type Properties = IconButtonToggleProps;

    fn create(_: &Context<Self>) -> Self {
        IconButtonToggle::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            change_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-icon-button-toggle
                 on={bool_to_option(props.on)}
                 onIcon={props.on_icon.clone()}
                 offIcon={props.off_icon.clone()}
                 label={props.label.clone()}
                 disabled={props.disabled}
                 ref={self.node_ref.clone()}
             > {props.children.clone()}</mwc-icon-button-toggle>
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clear event listener in case the props changed
        self.change_listener = None;
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        if self.change_listener.is_none() {
            let element = self.node_ref.cast::<IconButtonToggle>().unwrap();

            let callback = props.onchange.clone();
            self.change_listener = Some(EventListener::new(
                &element.clone(),
                "MDCIconButtonToggle:change",
                move |_| callback.emit(element.on()),
            ));
        }
    }
}
