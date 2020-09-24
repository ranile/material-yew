use yew::prelude::*;
use yew_material_components::{MatLinearProgress, MatButton};

pub struct LinearProgress {
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { closed: false, link, progress: 0.0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let buffer = self.progress - 0.3;

        html! {<>
            <section>
                <span onclick=self.link.callback(|_| Msg::Close)>
                    <MatButton label="Toggle" raised=true  />
                 </span><br />
                <div style="margin: 1em;">
                    <MatLinearProgress closed=self.closed progress=0.75 buffer=0.5 />
                </div>
            </section>

            <section>
                <h2>{"Indeterminate"}</h2>
                <div style="margin: 1em;">
                    <MatLinearProgress indeterminate=true />
                </div>
            </section>

            <section>
                <h2>{"Determinate"}</h2> <br />
                <span onclick=self.link.callback(|_| Msg::ChangeProgress)>
                    <MatButton label="Increase progress" raised=true />
                </span> <br />
                <div style="margin: 1em;">
                    <MatLinearProgress progress=self.progress buffer=buffer />
                </div>
            </section>
        </>}
    }
}
