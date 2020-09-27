use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use yew::web_sys::Node;
pub use web_sys::ValidityState as NativeValidityState;
use crate::{to_option, set_element_property};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum TextFieldType {
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DatetimeLocal,
    Number,
    Color,
}

impl ToString for TextFieldType {
    fn to_string(&self) -> String {
        use TextFieldType::*;
        match self {
            Text => "text",
            Search => "search",
            Tel => "tel",
            Url => "url",
            Email => "email",
            Password => "password",
            Date => "date",
            Month => "month",
            Week => "week",
            Time => "time",
            DatetimeLocal => "datetime-local",
            Number => "number",
            Color => "color",
        }.to_string()
    }
}

// TODO use snake_case
// TODO Figure out why the returned isn't doing anything
// The function is being called but the return value from that method means nothing
#[allow(non_snake_case)]
#[wasm_bindgen]
pub struct ValidityState {
    pub badInput: bool,
    pub customError: bool,
    pub patternMismatch: bool,
    pub rangeOverflow: bool,
    pub rangeUnderflow: bool,
    pub stepMismatch: bool,
    pub tooLong: bool,
    pub tooShort: bool,
    pub typeMismatch: bool,
    pub valid: bool,
    pub valueMissing: bool,
}

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextField;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = TextField)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(this: &TextField, val: &Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>);
}

// call the macro with the type
loader_hack!(TextField);

pub struct MatTextField {
    props: Props,
    node_ref: NodeRef,
    validity_transform_closure: Option<Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closing_closure: Option<Closure<dyn FnMut(JsValue)>>,
    closed_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

type ValidityTransformFn = fn(String, NativeValidityState) -> ValidityState;
#[derive(Clone)]
pub struct ValidityTransform(
    pub Rc<ValidityTransformFn>
);

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
    #[prop_or_default]
    pub onopening: Callback<()>,
}

impl Component for MatTextField {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextField::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), validity_transform_closure: None, opened_closure: None, closing_closure: None, closed_closure: None }
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
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
        if first_render {
            element.set_attribute("type", &self.props.field_type.to_string());

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

fn to_option_string(s: &str) -> Option<&str> {
    match s {
        "" => None,
        _ => Some(s)
    }
}
