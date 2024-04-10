use yew::prelude::*;

#[derive(PartialEq)]
pub enum FabVariants {
    Standard,
    Branded,
}

impl FabVariants {
    fn as_tag_name(&self) -> &'static str {
        match self {
            FabVariants::Standard => "md-fab",
            FabVariants::Branded => "md-branded-fab",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    /// The FAB color variant to render.
    #[prop_or(Some(AttrValue::Static("surface")))]
    pub kind: Option<AttrValue>,
    /// The size of the FAB.<br>NOTE: Branded FABs cannot be sized to <code>small</code>, and
    /// Extended FABs do not have different sizes.
    #[prop_or(Some(AttrValue::Static("medium")))]
    pub size: Option<AttrValue>,
    /// The text to display on the FAB.
    #[prop_or(Some(AttrValue::Static("")))]
    pub label: Option<AttrValue>,
    /// Lowers the FAB’s elevation.
    #[prop_or(Some(false))]
    pub lowered: Option<bool>,
    /// The variant to use.
    pub variant: FabVariants,
    pub children: Html,
}

#[function_component]
pub fn Fab(props: &Props) -> Html {
    use_effect_with((), |_| crate::import_material_web_module!("/md-web/fab.js"));
    html! { <@{props.variant.as_tag_name()}
        ~variant={crate::js_value_from_string_or_null(props.kind.as_ref())}
        ~size={crate::js_value_from_string_or_null(props.size.as_ref())}
        ~label={crate::js_value_from_string_or_null(props.label.as_ref())}
        ~lowered={crate::js_value_or_null(props.lowered.clone())}
    > {props.children.clone()} </@> }
}
