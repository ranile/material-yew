use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Progress to display, a fraction between 0 and <code>max</code>."]
    #[prop_or(Some(0))]
    pub value: Option<usize>,
    #[doc = "Maximum progress to display, defaults to 1."]
    #[prop_or(Some(1))]
    pub max: Option<usize>,
    #[doc = "Whether or not to display indeterminate progress, which gives no indication to how long an activity will take."]
    #[prop_or(Some(false))]
    pub indeterminate: Option<bool>,
    #[doc = "Whether or not to render indeterminate mode using 4 colors instead of one."]
    #[prop_or(Some(false))]
    pub four_color: Option<bool>,
    pub children: Html,
}

#[function_component]
pub fn CircularProgress(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/circular-progress.js")
    });
    html! { <md-circular-progress
        value={props.value.clone().unwrap_or_default()}
        ~max={crate::js_value_or_null(props.max.clone())}
        ~indeterminate={crate::js_value_or_null(props.indeterminate.clone())}
        ~fourColor={crate::js_value_or_null(props.four_color.clone())}
    > {props.children.clone()} </md-circular-progress> }
}
