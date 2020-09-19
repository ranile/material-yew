use yew::prelude::*;
use mat_web_comp::{MatCircularProgress, MatButton};

pub struct CircularProgress {
    closed: bool,
    progress: f32,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Close,
    ChangeProgress,
}

impl Component for CircularProgress {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CircularProgress { closed: false, link, progress: 0.0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => {
                self.closed = !self.closed;
            }
            Msg::ChangeProgress => {
                self.progress += 0.1;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {
        html! {
            <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">

                <div onclick=self.link.callback(|_| Msg::Close)>
                    <MatButton label="Close" raised=true  /> <br />
                    <MatCircularProgress indeterminate=true closed=self.closed />
                </div>

                <div onclick=self.link.callback(|_| Msg::ChangeProgress)>
                    <MatButton label="Increase progress" outlined=true /> <br />
                    <MatCircularProgress progress=self.progress />
                </div>
            </div>
        }
    }
}
