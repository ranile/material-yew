use crate::add_event_listener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-switch.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Switch;

    #[wasm_bindgen(getter, static_method_of = Switch)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn checked(this: &Switch) -> bool;

    #[wasm_bindgen(method, setter)]
    fn set_checked(this: &Switch, value: bool);
}

loader_hack!(Switch);

/// The `mwc-switch` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/switch)
pub struct MatSwitch {
    props: SwitchProps,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}

/// Props for [`MatSwitch`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/switch#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/switch#events)
#[derive(Debug, Properties, Clone)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// Binds to `change` event on `mwc-switch`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
}

impl Component for MatSwitch {
    type Message = ();
    type Properties = SwitchProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Switch::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            closure: None,
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
              <mwc-switch
                  disabled=self.props.disabled
                  ref=self.node_ref.clone()
              ></mwc-switch>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Switch>().unwrap();
        element.set_checked(self.props.checked);

        if first_render {
            let callback = self.props.onchange.clone();
            add_event_listener(
                &self.node_ref,
                "change",
                move || {
                    callback.emit(element.checked());
                },
                &mut self.closure,
            )
        }
    }
}
