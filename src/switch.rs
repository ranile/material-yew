use wasm_bindgen::JsValue;
use yew::prelude::*;
use web_sys::HtmlFormElement as HTMLFormElement;
use web_sys::NodeList;

type ValidityState = JsValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "Disables the switch and makes it non-interactive."]
    #[prop_or(Some(false))]
    pub disabled: Option<bool>,
    #[doc = "Puts the switch in the selected state and sets the form submission value to the <code>value</code> property."]
    #[prop_or(Some(false))]
    pub selected: Option<bool>,
    #[doc = "Shows both the selected and deselected icons."]
    #[prop_or(Some(false))]
    pub icons: Option<bool>,
    #[doc = "Shows only the selected icon, and not the deselected icon. If <code>true</code>, overrides the behavior of the <code>icons</code> property."]
    #[prop_or(Some(false))]
    pub show_only_selected_icon: Option<bool>,
    #[doc = "When true, require the switch to be selected when participating in form submission.<br>https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/checkbox#validation"]
    #[prop_or(Some(false))]
    pub required: Option<bool>,
    #[doc = "The value associated with this switch on form submission. <code>null</code> is submitted when <code>selected</code> is <code>false</code>."]
    #[prop_or(Some(AttrValue::Static("on")))]
    pub value: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub name: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub form: Option<HTMLFormElement>,
    #[doc = ""]
    #[prop_or(None)]
    pub labels: Option<NodeList>,
    #[doc = ""]
    #[prop_or(None)]
    pub validity: Option<ValidityState>,
    #[doc = ""]
    #[prop_or(None)]
    pub validation_message: Option<AttrValue>,
    #[doc = ""]
    #[prop_or(None)]
    pub will_validate: Option<bool>,
    #[prop_or(None)]
    pub oninput: Option<Callback<InputEvent>>,
    #[prop_or(None)]
    pub onchange: Option<Callback<Event>>,
}

#[function_component]
pub fn Switch(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/switch.js")
    });
    html! { <md-switch
        disabled={props.disabled.unwrap_or_default()}
        selected={props.selected.unwrap_or_default()}
        ~icons={crate::js_value_or_null(props.icons.clone())}
        ~showOnlySelectedIcon={crate::js_value_or_null(props.show_only_selected_icon.clone())}
        required={props.required.unwrap_or_default()}
        value={props.value.clone().unwrap_or_default()}
        ~name={crate::js_value_from_string_or_null(props.name.as_ref())}
        ~form={crate::js_value_or_null(props.form.clone())}
        ~labels={crate::js_value_or_null(props.labels.clone())}
        ~validity={crate::js_value_or_null(props.validity.clone())}
        ~validationMessage={crate::js_value_from_string_or_null(props.validation_message.as_ref())}
        ~willValidate={crate::js_value_or_null(props.will_validate.clone())}
        oninput={props.oninput.clone()}
        onchange={props.onchange.clone()}
    />}
}
