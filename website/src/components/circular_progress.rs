use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::{MatButton, MatCircularProgress, MatCircularProgressFourColor};
use yew::prelude::*;

pub struct CircularProgress {
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

    fn create(_: &Context<Self>) -> Self {
        Self {
            closed: false,
            progress: 0.0,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeProgress => {
                self.progress += 0.1;
            }
            Msg::Close => {
                self.closed = !self.closed;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let toggle_example = with_raw_code!(toggle_example { html! {
             <section class="demo">
                 <span onclick={link.callback(|_| Msg::Close)} >
                     <MatButton label="Toggle" raised=true  />
                  </span><br />
                 <MatCircularProgress closed={self.closed} progress=0.75 />
             </section>
        }});

        let indeterminate_ex = with_raw_code!(indeterminate_ex { html! {
             <MatCircularProgress indeterminate=true />
        }});

        let determinate_ex = with_raw_code!(determinate_ex { html! {
             <section class="demo">
                 <span onclick={link.callback(|_| Msg::ChangeProgress)} >
                     <MatButton label="Increase progress" raised=true />
                 </span>
                 <MatCircularProgress progress={self.progress} />
             </section>
        }});

        let four_color_ex = with_raw_code!(four_color_ex { html! {
             <MatCircularProgressFourColor indeterminate=true />
        }});

        html! {<>
            <Codeblock code_and_html={toggle_example} title="Toggle open state" />

            <Codeblock code_and_html={indeterminate_ex} title="Indeterminate" />

            <Codeblock code_and_html={determinate_ex} title="Determinate" />

            <p>
                <b>{"Note:"}</b> {" Four colored variants of circular progress is available under "} <code>{"MatCircularProgressFourColor"}</code>
            </p>

            <section class="four-colored-progress">
                <Codeblock code_and_html={four_color_ex} title="Four colored indeterminate" />
            </section>
        </>}
    }
}
