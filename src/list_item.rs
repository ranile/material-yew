use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Disables the item and makes it non-selectable and non-interactive."]
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "Sets the behavior of the list item, defaults to “text”. Change to “link” or “button” for interactive items."]
    #[prop_or(Some(AttrValue::Static("text")))]
    pub typepe: Option<AttrValue>,
    #[doc = "Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    #[doc = "Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when <code>href</code> is set."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    pub children: Html,

    #[prop_or(None)]
    pub onfocus: Option<Callback<FocusEvent>>,
}

#[function_component]
pub fn ListItem(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/list-item.js")
    });
    html! { <md-list-item
        disabled={props.disabled.unwrap_or_default()}
        ~type={crate::js_value_from_string_or_null(props.typepe.as_ref())}
        ~href={crate::js_value_from_string_or_null(props.href.as_ref())}
        ~target={crate::js_value_from_string_or_null(props.target.as_ref())}
        onfocus={props.onfocus.clone()}
    > {props.children.clone()} </md-list-item> }
}
