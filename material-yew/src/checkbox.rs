use crate::bool_to_option;
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-checkbox.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Checkbox;

    #[wasm_bindgen(getter, static_method_of = Checkbox)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Checkbox, value: bool);

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Checkbox) -> bool;
}

loader_hack!(Checkbox);

/// The `mwc-checkbox` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox)
pub struct MatCheckbox {
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatCheckbox`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#events)
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub reduced_touch_target: bool,
    /// Binds to `change` event on `mwc-checkbox`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatCheckbox {
    type Message = ();
    type Properties = CheckboxProps;

    fn create(_: &Context<Self>) -> Self {
        Checkbox::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            change_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
               <mwc-checkbox
                   indeterminate={bool_to_option(props.indeterminate)}
                   disabled={props.disabled}
                   value={props.value.clone()}
                   reducedTouchTarget={bool_to_option(props.reduced_touch_target)}
                   ref={self.node_ref.clone()}
               ></mwc-checkbox>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Checkbox>().unwrap();
        element.set_checked(props.checked);

        if self.change_listener.is_none() {
            let callback = props.onchange.clone();
            let target = self.node_ref.cast::<Element>().unwrap();
            self.change_listener = Some(EventListener::new(&target, "change", move |_| {
                callback.emit(element.checked());
            }));
        }
    }
}
