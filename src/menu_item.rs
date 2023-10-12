use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Disables the item and makes it non-selectable and non-interactive."]
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "Sets the behavior and role of the menu item, defaults to “menuitem”."]
    #[prop_or(Some(AttrValue::Static("menuitem")))]
    pub typepe: Option<AttrValue>,
    #[doc = "Sets the underlying <code>HTMLAnchorElement</code>’s <code>href</code> resource attribute."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub href: Option<AttrValue>,
    #[doc = "Sets the underlying <code>HTMLAnchorElement</code>’s <code>target</code> attribute when <code>href</code> is set."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub target: Option<AttrValue>,
    #[doc = "Keeps the menu open if clicked or keyboard selected."]
    #[prop_or(Some(false))]
    pub keep_open: Option<bool>,
    #[doc = "Sets the item in the selected visual state when a submenu is opened."]
    #[prop_or(Some(false))]
    pub selected: Option<bool>,
    #[doc = ""]
    # [prop_or (Some (AttrValue :: Static ("")))]
    pub typepeahead_text: Option<AttrValue>,
    pub children: Html,
}

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/menu-item.js")
    });
    html! { <md-menu-item
       disabled={props.disabled.unwrap_or_default()}
       ~type={crate::js_value_from_string_or_null(props.typepe.as_ref())}
       ~href={crate::js_value_from_string_or_null(props.href.as_ref())}
       ~target={crate::js_value_from_string_or_null(props.target.as_ref())}
       ~keepOpen={crate::js_value_or_null(props.keep_open.clone())}
       selected={props.selected.unwrap_or_default()}
       ~typeaheadText={crate::js_value_from_string_or_null(props.typepeahead_text.as_ref())}
    >
        {props.children.clone()}
    </md-menu-item> }
}
