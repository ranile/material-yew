use super::set_on_input_handler;
use crate::bool_to_option;
use crate::text_inputs::validity_state::ValidityStateJS;
use crate::text_inputs::{TextFieldType, ValidityState, ValidityTransform};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
pub use web_sys::ValidityState as NativeValidityState;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-textarea.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextArea;

    #[wasm_bindgen(getter, static_method_of = TextArea)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextArea,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextArea, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextArea) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextArea, val: &JsValue);
}

loader_hack!(TextArea);

/// The `mwc-textarea` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/textarea)
pub struct MatTextArea {
    node_ref: NodeRef,
    validity_transform_closure:
        Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    input_listener: Option<EventListener>,
}

/// Type for [`TextAreaProps::char_counter`].
///
/// Equivalent to `type TextAreaCharCounter = 'external'|'internal';` Typescript
/// type.
#[derive(Clone, Copy, PartialEq)]
pub enum TextAreaCharCounter {
    Internal,
    External,
}

impl TextAreaCharCounter {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextAreaCharCounter::Internal => "internal",
            TextAreaCharCounter::External => "external",
        }
    }
}

/// Props for [`MatTextArea`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/checkbox#propertiesattributes)
#[derive(Properties, PartialEq, Clone)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub rows: Option<i64>,
    #[prop_or_default]
    pub cols: Option<i64>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or(TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    #[prop_or_default]
    pub icon_trailing: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    /// For boolean value `true`, `TextAreaCharCounter::External` is to be used.
    /// Boolean value `false` results in character counter not being shown so
    /// `None` should be used
    #[prop_or_default]
    pub char_counter: Option<TextAreaCharCounter>,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: Option<AttrValue>,
    #[prop_or_default]
    pub helper_persistent: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub max_length: Option<u64>,
    #[prop_or_default]
    pub validation_message: Option<AttrValue>,
    /// Type: `number | string` so I'll leave it as a string
    #[prop_or_default]
    pub min: Option<AttrValue>,
    /// Type: `number | string`  so I'll leave it as a string
    #[prop_or_default]
    pub max: Option<AttrValue>,
    #[prop_or_default]
    pub size: Option<i64>, // --|
    #[prop_or_default] //   | -- What you doing step size
    pub step: Option<i64>, // --|
    #[prop_or_default]
    pub auto_validate: bool,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub oninput: Callback<String>,
    #[prop_or_default]
    pub name: Option<AttrValue>,
}

impl Component for MatTextArea {
    type Message = ();
    type Properties = TextAreaProps;

    fn create(_: &Context<Self>) -> Self {
        TextArea::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
            input_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-textarea
                 rows={props.rows.map(|v| v.to_string())}
                 cols={props.cols.map(|v| v.to_string())}
                 label={props.label.clone()}
                 placeholder={props.placeholder.clone()}
                 icon={props.icon.clone()}
                 iconTrailing={props.icon_trailing.clone()}
                 disabled={props.disabled}
                 charCounter={props.char_counter.map(|it| it.as_str())}
                 outlined={bool_to_option(props.outlined)}
                 helper={props.helper.clone()}
                 helperPersistent={bool_to_option(props.helper_persistent)}
                 required={props.required}
                 maxLength={props.max_length.map(|v| v.to_string())}
                 validationMessage={props.validation_message.clone()}
                 min={props.min.clone()}
                 max={props.max.clone()}
                 size={props.size.map(|v| v.to_string())}
                 step={props.step.map(|v| v.to_string())}
                 autoValidate={bool_to_option(props.auto_validate)}
                 validateOnInitialRender={bool_to_option(props.validate_on_initial_render)}
                 name={props.name.clone()}
                 ref={self.node_ref.clone()}
             ></mwc-textarea>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<TextArea>().unwrap();
        element.set_type(&JsValue::from(props.field_type.as_str()));
        element.set_value(&JsValue::from_str(
            props.value.as_ref().map(|s| s.as_ref()).unwrap_or_default(),
        ));

        if self.input_listener.is_none() {
            self.input_listener = Some(set_on_input_handler(
                &self.node_ref,
                props.oninput.clone(),
                |(_, detail)| {
                    detail
                        .unchecked_into::<MatTextAreaInputEvent>()
                        .target()
                        .value()
                },
            ));
        };

        if first_render {
            let this = self.node_ref.cast::<TextArea>().unwrap();
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

impl MatTextArea {
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}

#[wasm_bindgen]
extern "C" {
    type MatTextAreaInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextAreaInputEvent) -> TextArea;
}
