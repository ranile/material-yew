use yew::prelude::*;
use yew_material_components::{MatCircularProgress, MatCircularProgressFourColor, MatButton};
use crate::components::Codeblock;
use crate::with_raw_code;

pub struct CircularProgress {
    link: ComponentLink<Self>,
    closed: bool,
    progress: f32,
}

pub enum Msg {
    Close,
    ChangeProgress,
}

impl Component for CircularProgress {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { closed: false, link, progress: 0.0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeProgress => {
                self.progress += 0.1;
            }
            Msg::Close => {
                yew::services::ConsoleService::log("test");
                self.closed = !self.closed;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {

        let toggle_example = with_raw_code!(toggle_example { html! {
            <section class="demo">
                <span onclick=self.link.callback(|_| Msg::Close)>
                    <MatButton label="Toggle" raised=true  />
                 </span><br />
                <MatCircularProgress closed=self.closed progress=0.75 />
            </section>
        }});

        let indeterminate_ex = with_raw_code!(indeterminate_ex { html! {
            <MatCircularProgress indeterminate=true />
        }});

        let determinate_ex = with_raw_code!(determinate_ex { html! {
            <section class="demo">
                <span onclick=self.link.callback(|_| Msg::ChangeProgress)>
                    <MatButton label="Increase progress" raised=true />
                </span>
                <MatCircularProgress progress=self.progress />
            </section>
        }});

        let four_color_ex = with_raw_code!(four_color_ex { html! {
            <MatCircularProgressFourColor indeterminate=true />
        }});

        html! {<>
            <Codeblock code_and_html=toggle_example title="Toggle open state" />

            <Codeblock code_and_html=indeterminate_ex title="Indeterminate" />

            <Codeblock code_and_html=determinate_ex title="Determinate" />

            <p>
                <b>{"Note:"}</b> {" Four colored variants of circular progress is available under "} <code>{"MatCircularProgressFourColor"}</code>
            </p>

            <section class="four-colored-progress">
                <Codeblock code_and_html=four_color_ex title="Four colored indeterminate" />
            </section>
        </>}
    }
}
