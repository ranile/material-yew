use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener};
use web_sys::Node;

#[wasm_bindgen(module = "/build/built-js.js")]
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

pub struct MatCheckbox {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}

/// Props for [`MatCheckbox`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#events)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub value: String,
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
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Checkbox::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
              <mwc-checkbox
                  indeterminate?=to_option(self.props.indeterminate)
                  disabled=self.props.disabled
                  value=self.props.value
                  reducedTouchTarget?=to_option(self.props.reduced_touch_target)
                  ref=self.node_ref.clone()
              ></mwc-checkbox>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Checkbox>().unwrap();
        if self.props.checked {
            element.set_checked(self.props.checked);
        }
        if first_render {
            let callback = self.props.onchange.clone();
            add_event_listener(&self.node_ref, "change", move || {
                callback.emit(element.checked());
            }, &mut self.closure)
        }
    }
}
