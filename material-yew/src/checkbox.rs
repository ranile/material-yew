use crate::bool_to_option;
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;

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
    props: CheckboxProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatCheckbox`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#events)
#[derive(Debug, Properties, Clone)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: Cow<'static, str>,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Checkbox::ensure_loaded();
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
              <mwc-checkbox
                  indeterminate=bool_to_option(self.props.indeterminate)
                  disabled=self.props.disabled
                  value=self.props.value.clone()
                  reducedTouchTarget=bool_to_option(self.props.reduced_touch_target)
                  ref=self.node_ref.clone()
              ></mwc-checkbox>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let element = self.node_ref.cast::<Checkbox>().unwrap();
        element.set_checked(self.props.checked);

        if self.change_listener.is_none() {
            let callback = self.props.onchange.clone();
            let target = self.node_ref.cast::<Element>().unwrap();
            self.change_listener = Some(EventListener::new(&target, "change", move |_| {
                callback.emit(element.checked());
            }));
        }
    }
}
