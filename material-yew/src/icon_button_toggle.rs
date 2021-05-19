mod off_icon;
mod on_icon;

pub use off_icon::*;
pub use on_icon::*;

use crate::bool_to_option;
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle)
pub struct MatIconButtonToggle {
    props: IconButtonToggleProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatIconButtonToggle`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#events)
#[derive(Debug, Properties, Clone)]
pub struct IconButtonToggleProps {
    #[prop_or_default]
    pub on: bool,
    #[prop_or_default]
    pub on_icon: Cow<'static, str>,
    #[prop_or_default]
    pub off_icon: Cow<'static, str>,
    #[prop_or_default]
    pub label: Cow<'static, str>,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        IconButtonToggle::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            change_listener: None,
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
            <mwc-icon-button-toggle
                on=bool_to_option(self.props.on)
                onIcon=self.props.on_icon.clone()
                offIcon=self.props.off_icon.clone()
                label=self.props.label.clone()
                disabled=self.props.disabled
                ref=self.node_ref.clone()
            > { self.props.children.clone() }</mwc-icon-button-toggle>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.change_listener.is_none() {
            let element = self.node_ref.cast::<IconButtonToggle>().unwrap();

            let callback = self.props.onchange.clone();
            self.change_listener = Some(EventListener::new(
                &element.clone(),
                "MDCIconButtonToggle:change",
                move |_| callback.emit(element.on()),
            ));
        }
    }
}
