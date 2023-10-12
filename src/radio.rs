use yew::prelude::*;
use web_sys::HtmlFormElement as HTMLFormElement;
use web_sys::NodeList;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Whether or not the radio is disabled."]
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "The element value to use in form submission when checked."]
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub checked: Option<bool>,
    #[doc = ""]
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    #[doc = ""]
    #[prop_or(None)]
    pub labels: Option<NodeList>,
    pub children: Html,
}

#[function_component]
pub fn Radio(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/radio.js")
    });
    html! { <md-radio
        disabled={props.disabled.unwrap_or_default()}
        value={props.value.clone().unwrap_or_default()}
        ~checked={crate::js_value_or_null(props.checked.clone())}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
    > {props.children.clone()} </md-radio> }
}
