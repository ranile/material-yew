use super::set_on_input_handler;
use crate::bool_to_option;
use crate::text_inputs::{
    validity_state::ValidityStateJS, TextFieldType, ValidityState, ValidityTransform,
};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use web_sys::ValidityState as NativeValidityState;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/mwc-textfield.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextField;

    #[wasm_bindgen(getter, static_method_of = TextField)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextField,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextField, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextField) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextField, val: &JsValue);
}

loader_hack!(TextField);

/// The `mwc-textfield` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield)
pub struct MatTextField {
    node_ref: NodeRef,
    validity_transform_closure:
        Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    input_listener: Option<EventListener>,
}

/// Props for [`MatTextField`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/textfield#propertiesattributes)
#[derive(Properties, PartialEq, Clone)]
pub struct TextFieldProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub value: String,
    #[prop_or(TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub prefix: String,
    #[prop_or_default]
    pub suffix: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub icon_trailing: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub char_counter: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: String,
    #[prop_or_default]
    pub helper_persistent: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub max_length: Option<u64>,
    #[prop_or_default]
    pub validation_message: String,
    #[prop_or_default]
    pub pattern: String,
    /// Type: `number | string` so I'll leave it as a string
    #[prop_or_default]
    pub min: String,
    /// Type: `number | string`  so I'll leave it as a string
    #[prop_or_default]
    pub max: String,
    // What you doing...
    #[prop_or_default]
    pub size: Option<i64>,
    // ...step size
    #[prop_or_default]
    pub step: Option<i64>,
    #[prop_or_default]
    pub auto_validate: bool,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub name: String,
}

impl Component for MatTextField {
    type Message = ();
    type Properties = TextFieldProps;

    fn create(_: &Context<Self>) -> Self {
        TextField::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
            input_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-textfield
                 open={props.open}
                 label={props.label.clone()}
                 placeholder={props.placeholder.clone()}
                 prefix={props.prefix.clone()}
                 suffix={props.suffix.clone()}
                 icon={props.icon.clone()}
                 iconTrailing={props.icon_trailing.clone()}
                 disabled={props.disabled}
                 charCounter={bool_to_option(props.char_counter)}
                 outlined={bool_to_option(props.outlined)}
                 helper={props.helper.clone()}
                 helperPersistent={bool_to_option(props.helper_persistent)}
                 required={props.required}
                 maxLength={props.max_length.map(|v| v.to_string())}
                 validationMessage={props.validation_message.clone()}
                 pattern={props.pattern.clone()}
                 min={props.min.clone()}
                 max={props.max.clone()}
                 size={props.size.map(|v| v.to_string())}
                 step={props.step.map(|v| v.to_string())}
                 autoValidate={bool_to_option(props.auto_validate)}
                 validateOnInitialRender={bool_to_option(props.validate_on_initial_render)}
                 name={props.name.clone()}
                 ref={self.node_ref.clone()}
             ></mwc-textfield>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<TextField>().unwrap();
        element.set_type(&JsValue::from(props.field_type.as_str()));
        element.set_value(&JsValue::from_str(props.value.as_ref()));

        if self.input_listener.is_none() {
            self.input_listener = Some(set_on_input_handler(
                &self.node_ref,
                props.oninput.clone(),
                |(_, detail)| {
                    detail
                        .unchecked_into::<MatTextFieldInputEvent>()
                        .target()
                        .value()
                },
            ));
        }
        if first_render {
            let this = self.node_ref.cast::<TextField>().unwrap();
            if let Some(transform) = props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(
                    move |s: String, v: NativeValidityState| -> ValidityStateJS {
                        transform.0(s, v).into()
                    },
                )
                    as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                this.set_validity_transform(self.validity_transform_closure.as_ref().unwrap());
            }
        }
    }
}

impl MatTextField {
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}

#[wasm_bindgen]
extern "C" {
    type MatTextFieldInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextFieldInputEvent) -> TextField;
}
