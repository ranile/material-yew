use wasm_bindgen::prelude::*;
use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;
use yew::virtual_dom::VTag;

macro_rules! import_material_web_module {
    ($module:literal) => {{
        #[wasm_bindgen::prelude::wasm_bindgen(module = $module)]
        extern "C" {
            #[wasm_bindgen(getter)]
            fn __dummy_loader() -> wasm_bindgen::JsValue;
        }

        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        {
            LOADED.call_once(|| {
                __dummy_loader();
            });
        }
    }};
}

fn js_value_or_null<T>(v: Option<T>) -> JsValue
where
    JsValue: From<T>,
{
    match v {
        Some(v) => JsValue::from(v),
        None => JsValue::NULL,
    }
}

fn js_value_from_string_or_null(v: Option<&AttrValue>) -> JsValue {
    match v {
        Some(v) => JsValue::from_str(&*v),
        None => JsValue::NULL,
    }
}

#[derive(PartialEq)]
pub enum ButtonVariants {
    Elevated,
    Filled,
    FilledTonal,
    Outlined,
    Text,
}

impl ButtonVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            ButtonVariants::Elevated => "md-elevated-button",
            ButtonVariants::Filled => "md-filled-button",
            ButtonVariants::FilledTonal => "md-filled-tonal-button",
            ButtonVariants::Outlined => "md-outlined-button",
            ButtonVariants::Text => "md-text-button",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Whether or not the button is disabled."]
    #[prop_or_default]
    pub disabled: Option<bool>,
    #[doc = "The URL that the link button points to."]
    #[prop_or_default]
    pub href: Option<AttrValue>,
    #[doc = "Where to display the linked <code>href</code> URL for a link button. Common options include <code>_blank</code> to open in a new tab."]
    #[prop_or_default]
    pub target: Option<AttrValue>,
    #[doc = "Whether to render the icon at the inline end of the label rather than the inline start.<br><em>Note:</em> Link buttons cannot have trailing icons."]
    #[prop_or_default]
    pub trailing_icon: Option<bool>,
    #[doc = "Whether to display the icon or not."]
    #[prop_or_default]
    pub has_icon: Option<bool>,
    #[doc = ""]
    #[prop_or_default]
    pub typepe: Option<AttrValue>,
    #[doc = ""]
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[doc = ""]
    #[prop_or_default]
    pub form: Option<HTMLFormElement>,
    #[doc = "The variant to use."]
    pub variant: ButtonVariants,
    pub children: Html,
}

mod js {
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "import")]
    fn import(v: &str) -> JsValue;
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    use_effect_with((), |_| import_material_web_module!("/build/dist/button.js"));
    html! { <@{props.variant.as_tag_name()}
           disabled={props.disabled.unwrap_or_default()}
           ~href={js_value_from_string_or_null(props.href.as_ref())}
           ~target={js_value_from_string_or_null(props.target.as_ref())}
           ~trailingIcon={js_value_or_null(props.trailing_icon.clone())}
           ~hasIcon={js_value_or_null(props.has_icon.clone())}
           ~type={js_value_from_string_or_null(props.typepe.as_ref())}
           ~value={props.value.clone().unwrap_or_default()}
           ~name={js_value_from_string_or_null(props.name.as_ref())}
           ~form={js_value_or_null(props.form.clone())}
    > {props.children.clone()} </@> }
}
