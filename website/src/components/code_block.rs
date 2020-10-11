use yew::prelude::*;
use yew_material_components::MatIconButton;
use crate::{html_to_element};

pub struct Codeblock {
    link: ComponentLink<Self>,
    props: Props,
    showing_code: bool,
}

pub enum Msg {
    FlipShowCode,
}

#[derive(Properties, Clone)]
pub struct Props {
    // pub children: Children,
    // pub code: String,
    pub title: String,
    pub code_and_html: (String, Html),
    #[prop_or(45)]
    pub max_width: u32
}

impl Component for Codeblock {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, showing_code: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FlipShowCode => {
                self.showing_code = !self.showing_code;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let code = html_to_element(&self.props.code_and_html.0);
        html! { <>
            <section class="codeblock" style=format!("max-width: {}%", self.props.max_width)>
                <section class="header">
                    <h2 class="title">{&self.props.title}</h2>
                    <span class="right-icon" onclick=self.link.callback(|_| Msg::FlipShowCode)>
                        <MatIconButton icon="code" />
                    </span>
                </section>

                {
                    if self.showing_code {
                        {code}
                    } else { html!{} }
                }

                { self.props.code_and_html.1.clone() }
            </section>
        </> }
    }
}
