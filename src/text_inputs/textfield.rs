use wasm_bindgen::prelude::*;
use yew::prelude::*;
pub use web_sys::ValidityState as NativeValidityState;
use crate::{to_option, to_option_string};
use crate::text_inputs::{ValidityState, TextFieldType, ValidityTransform, ValidityTransformFn};
use std::rc::Rc;
use web_sys::Node;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextField;

    #[wasm_bindgen(getter, static_method_of = TextField)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(this: &TextField, val: &Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>);

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextField, val: &JsValue);
}

loader_hack!(TextField);

pub struct MatTextField {
    props: Props,
    node_ref: NodeRef,
    validity_transform_closure: Option<Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>>,
}

#[derive(Properties, Clone)]
pub struct Props {
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
    pub max_length: String,
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
    pub name: String,
}

impl Component for MatTextField {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextField::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), validity_transform_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-textfield
                open=self.props.open
                value?=to_option_string(&self.props.value)
                label?=to_option_string(&self.props.label)
                placeholder?=to_option_string(&self.props.placeholder)
                prefix?=to_option_string(&self.props.prefix)
                suffix?=to_option_string(&self.props.suffix)
                icon?=to_option_string(&self.props.icon)
                iconTrailing?=to_option_string(&self.props.icon_trailing)
                disabled=self.props.disabled
                charCounter?=to_option(self.props.char_counter)
                outlined?=to_option(self.props.outlined)
                helper?=to_option_string(&self.props.helper)
                helperPersistent?=to_option(self.props.helper_persistent)
                required=self.props.required
                maxLength?=to_option_string(&self.props.max_length)
                validationMessage?=to_option_string(&self.props.validation_message)
                pattern?=to_option_string(&self.props.pattern)
                min?=to_option_string(&self.props.min)
                max?=to_option_string(&self.props.max)
                size?=self.props.size //.map_or("null".to_string(), |v| v.to_string())
                step?=self.props.step //.map_or("null".to_string(), |v| v.to_string())
                autoValidate?=to_option(self.props.auto_validate)
                validateOnInitialRender?=to_option(self.props.validate_on_initial_render)
                name?=to_option_string(&self.props.name)
                ref=self.node_ref.clone()
            ></mwc-textfield>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.node_ref.cast::<TextField>().unwrap();
            element.set_type(&JsValue::from(&self.props.field_type.to_string()));

            let this = self.node_ref.cast::<TextField>().unwrap();
            if let Some(c) = &self.props.validity_transform {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(*c.0) as Box<dyn FnMut(String, NativeValidityState) -> ValidityState>));
                this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }
        }
    }
}

impl MatTextField {
    pub fn validity_transform(func: ValidityTransformFn) -> ValidityTransform {
        ValidityTransform(Rc::new(func))
    }
}
