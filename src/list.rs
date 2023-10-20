use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
pub fn List(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/list.js")
    });
    html! { <md-list
    > {props.children.clone()} </md-list> }
}
