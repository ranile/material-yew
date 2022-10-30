use crate::bool_to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-radio.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Radio;

    #[wasm_bindgen(getter, static_method_of = Radio)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Radio) -> bool;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Radio, value: bool);
}

loader_hack!(Radio);

/// The `mwc-radio` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/radio)
pub struct MatRadio {
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatRadio`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/radio#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/radio#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct RadioProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub global: bool,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    /// Binds to `change`.
    ///
    /// Callback's parameter of type denotes if the radio is checked or not.
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatRadio {
    type Message = ();
    type Properties = RadioProps;

    fn create(_: &Context<Self>) -> Self {
        Radio::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            change_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
               <mwc-radio
                   disabled={props.disabled}
                   name={props.name.clone()}
                   value={props.value.clone()}
                   global={bool_to_option(props.global)}
                   reducedTouchTarget={bool_to_option(props.reduced_touch_target)}
                   ref={self.node_ref.clone()}
               ></mwc-radio>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Radio>().unwrap();
        element.set_checked(props.checked);

        if self.change_listener.is_none() {
            let callback = props.onchange.clone();
            self.change_listener =
                Some(EventListener::new(&element.clone(), "change", move |_| {
                    callback.emit(element.checked());
                }));
        }
    }
}
