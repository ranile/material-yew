use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::{MatButton, MatLinearProgress};
use yew::prelude::*;

pub struct LinearProgress {
    closed: bool,
    progress: f32,
}

pub enum Msg {
    Close,
    ChangeProgress,
}

impl Component for LinearProgress {
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
        let buffer = self.progress - 0.3;

        let toggle = with_raw_code!(toggle { html! {
         <section>
             <span onclick={link.callback(|_| Msg::Close)}>
                 <MatButton label="Toggle" raised=true  />
              </span><br />
             <div style="margin: 1em;">
                 <MatLinearProgress closed={self.closed} progress=0.75 buffer=0.5 />
             </div>
         </section>
        }});

        let indeterminate = with_raw_code!(indeterminate { html! {
         <section>
             <div style="margin: 1em;">
                 <MatLinearProgress indeterminate=true />
             </div>
         </section>
        }});

        let determinate = with_raw_code!(determinate { html! {
         <section>
             <span onclick={link.callback(|_| Msg::ChangeProgress)}>
                 <MatButton label="Increase progress" raised=true />
             </span> <br />
             <div style="margin: 1em;">
                 <MatLinearProgress progress={self.progress} buffer={buffer} />
             </div>
         </section>
        }});
        html! {<>
            <Codeblock title="Toggle Linear Progress" code_and_html={toggle} />

            <Codeblock title="Indeterminate Linear Progress" code_and_html={indeterminate} />

            <Codeblock title="Determinate Linear Progress" code_and_html={determinate} />
        </>}
    }
}
