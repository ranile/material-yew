use web_sys::HtmlFormElement as HTMLFormElement;
use yew::prelude::*;

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
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "The URL that the link button points to."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    #[doc = "Where to display the linked <code>href</code> URL for a link button. Common options include <code>_blank</code> to open in a new tab."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    #[doc = "Whether to render the icon at the inline end of the label rather than the inline start.<br><em>Note:</em> Link buttons cannot have trailing icons."]
    #[prop_or(Some(false))]
    pub trailing_icon: Option<bool>,
    #[doc = "Whether to display the icon or not."]
    #[prop_or(Some(false))]
    pub has_icon: Option<bool>,
    #[doc = ""]
    #[prop_or(Some(AttrValue::Static("submit")))]
    pub typepe: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(Some(AttrValue::Static("")))]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(Some(AttrValue::Static("")))]
    pub name: Option<AttrValue>,
    #[doc = ""]
    # [prop_or (Some (wasm_bindgen::JsValue::NULL . into ()))]
    pub form: Option<HTMLFormElement>,
    #[doc = "The variant to use."]
    pub variant: ButtonVariants,
    pub children: Html,
    #[prop_or(None)]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/button.js")
    });
    html! { <@{props.variant.as_tag_name()}
        disabled={props.disabled.unwrap_or_default()}
        ~href={crate::js_value_from_string_or_null(props.href.as_ref())}
        ~target={crate::js_value_from_string_or_null(props.target.as_ref())}
        ~trailingIcon={crate::js_value_or_null(props.trailing_icon.clone())}
        ~hasIcon={crate::js_value_or_null(props.has_icon.clone())}
        ~type={crate::js_value_from_string_or_null(props.typepe.as_ref())}
        value={props.value.clone().unwrap_or_default()}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        onclick={props.onclick.clone()}
    > {props.children.clone()} </@> }
}
