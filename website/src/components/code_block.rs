use yew::prelude::*;

pub struct CodeBlock {
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    props: Props,
}

pub enum Msg {

}

#[derive(Properties, Clone)]
pub struct Props {
    children: Children
}

impl Component for CodeBlock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, node_ref: NodeRef::default() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! { <>
            { self.props.children.clone() }
        </> }
    }
}
