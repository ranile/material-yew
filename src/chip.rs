use yew::prelude::*;

#[derive(PartialEq)]
pub enum ChipVariants {
    Assist,
    Filter,
    Input,
    Suggestion,
}

impl ChipVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            ChipVariants::Assist => "md-assist-chip",
            ChipVariants::Filter => "md-filter-chip",
            ChipVariants::Input => "md-input-chip",
            ChipVariants::Suggestion => "md-suggestion-chip",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    ///
    #[prop_or(Some(false))]
    pub elevated: Option<bool>,
    ///
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    ///
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    /// Whether or not the chip is disabled.<br>Disabled chips are not focusable, unless
    /// <code>always-focusable</code> is set.
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    /// When true, allow disabled chips to be focused with arrow keys.<br>Add this when a chip needs increased visibility when disabled. See https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/#kbd_disabled_controls for more guidance on when this is needed.
    #[prop_or(Some(false))]
    pub always_focusable: Option<bool>,
    /// The label of the chip.
    #[prop_or(Some(AttrValue::Static("")))]
    pub label: Option<AttrValue>,
    /// The variant to use.
    pub variant: ChipVariants,
    pub children: Html,
}

#[function_component]
pub fn Chip(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/chip.js")
    });
    html! { <@{props.variant.as_tag_name()}
        ~elevated={crate::js_value_or_null(props.elevated.clone())}
        ~href={crate::js_value_from_string_or_null(props.href.as_ref())}
        ~target={crate::js_value_from_string_or_null(props.target.as_ref())}
        disabled={props.disabled.unwrap_or_default()}
        ~alwaysFocusable={crate::js_value_or_null(props.always_focusable.clone())}
        ~label={crate::js_value_from_string_or_null(props.label.as_ref())}
    > {props.children.clone()} </@> }
}
