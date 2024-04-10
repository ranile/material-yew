use web_sys::{HtmlFormElement as HTMLFormElement, NodeList};
use yew::prelude::*;
#[derive(PartialEq)]
pub enum IconButtonVariants {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

impl IconButtonVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            IconButtonVariants::Standard => "md-icon-button",
            IconButtonVariants::Filled => "md-filled-icon-button",
            IconButtonVariants::FilledTonal => "md-filled-tonal-icon-button",
            IconButtonVariants::Outlined => "md-outlined-icon-button",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// Disables the icon button and makes it non-interactive.
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    /// Flips the icon if it is in an RTL context at startup.
    #[prop_or(Some(false))]
    pub flip_icon_in_rtl: Option<bool>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute.
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    /// Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute.
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    /// The <code>aria-label</code> of the button when the button is toggleable and selected.
    #[prop_or(Some(AttrValue::Static("")))]
    pub aria_label_selected: Option<AttrValue>,
    /// When true, the button will toggle between selected and unselected states
    #[prop_or(Some(false))]
    pub toggle: Option<bool>,
    /// Sets the selected state. When false, displays the default icon. When true, displays the
    /// selected icon, or the default icon If no <code>slot=&quot;selected&quot;</code> icon is
    /// provided.
    #[prop_or(Some(false))]
    pub selected: Option<bool>,
    ///
    #[prop_or(Some(AttrValue::Static("submit")))]
    pub typepe: Option<AttrValue>,
    ///
    #[prop_or(Some(AttrValue::Static("")))]
    pub value: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    ///
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    ///
    #[prop_or(None)]
    pub labels: Option<NodeList>,
    /// The variant to use.
    pub variant: IconButtonVariants,
    pub children: Html,

    #[prop_or(None)]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or(None)]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or(None)]
    pub onchange: Option<Callback<Event>>,
}

#[function_component]
pub fn IconButton(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/icon-button.js")
    });
    html! { <@{props.variant.as_tag_name()}
        disabled={props.disabled.unwrap_or_default()}
        ~flipIconInRtl={crate::js_value_or_null(props.flip_icon_in_rtl.clone())}
        ~href={crate::js_value_from_string_or_null(props.href.as_ref())}
        ~target={crate::js_value_from_string_or_null(props.target.as_ref())}
        ~ariaLabelSelected={crate::js_value_from_string_or_null(props.aria_label_selected.as_ref())}
        ~toggle={crate::js_value_or_null(props.toggle.clone())}
        selected={props.selected.unwrap_or_default()}
        ~type={crate::js_value_from_string_or_null(props.typepe.as_ref())}
        value={props.value.clone().unwrap_or_default()}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
        onclick={props.onclick.clone()}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    > {props.children.clone()} </@> }
}
