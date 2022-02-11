use crate::html_to_element;
use material_yew::MatIconButton;
use yew::prelude::*;

pub struct Codeblock {
    showing_code: bool,
}

pub enum Msg {
    FlipShowCode,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    // pub children: Children,
    // pub code: String,
    pub title: String,
    pub code_and_html: (String, Html),
    #[prop_or(45)]
    pub max_width: u32,
}

impl Component for Codeblock {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            showing_code: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FlipShowCode => {
                self.showing_code = !self.showing_code;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let code = html_to_element(&props.code_and_html.0);
        html! { <>
            <section class="codeblock" style={format!("max-width: {}%", props.max_width)}>
                <section class="header">
                    <h2 class="title">{&props.title}</h2>
                    <span class="right-icon" onclick={link.callback(|_| Msg::FlipShowCode)}>
                        <MatIconButton icon="code" />
                    </span>
                </section>

                {
                    if self.showing_code {
                        {code}
                   } else { html!{}}
               }

                {props.code_and_html.1.clone()}
            </section>
        </>}
    }
}
