use crate::bool_to_option;
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/radio)
pub struct MatRadio {
    props: RadioProps,
    node_ref: NodeRef,
    change_listener: Option<EventListener>,
}

/// Props for [`MatRadio`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/radio#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/radio#events)
#[derive(Debug, Properties, Clone)]
pub struct RadioProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub name: Cow<'static, str>,
    #[prop_or_default]
    pub value: Cow<'static, str>,
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

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Radio::ensure_loaded();
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
              <mwc-radio
                  disabled=self.props.disabled
                  name=self.props.name.clone()
                  value=self.props.value.clone()
                  global=bool_to_option(self.props.global)
                  reducedTouchTarget=bool_to_option(self.props.reduced_touch_target)
                  ref=self.node_ref.clone()
              ></mwc-radio>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Radio>().unwrap();
        element.set_checked(self.props.checked);

        if first_render {
            let callback = self.props.onchange.clone();
            self.change_listener =
                Some(EventListener::new(&element.clone(), "change", move |_| {
                    callback.emit(element.checked());
                }));
        }
    }
}
