use yew::prelude::*;
use mat_web_comp::{MatFormfield, MatCheckbox, MatDialog, MatButton};
use yew::services::ConsoleService;

pub struct Dialog {
    link: ComponentLink<Self>,
    open: bool
}

pub enum Msg {
    Close,
    Open,
    Event(String)
}

impl Component for Dialog {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, open: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Close => self.open = false,
            Msg::Open => self.open = true,
            Msg::Event(e) => ConsoleService::log(&format!("Got event {}", e))
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {

        html! {
        <div>
            <span onclick=self.link.callback(|_| Msg::Open)>
                <MatButton label="Open" raised=true />
            </span>

            <MatDialog open=self.open
                onopening=self.link.callback(|_| Msg::Event("onopening".to_string()))
                onopened=self.link.callback(|_| Msg::Event("onopened".to_string()))
                onclosing=self.link.callback(|_| Msg::Event("onclosing".to_string()))
                onclosed=self.link.callback(|_| Msg::Event("onclosed".to_string()))>
                <div>{"Discard draft?"}</div>
                <span slot="primaryAction" dialogAction="discard"><MatButton label="Discard"/></span>
                <span slot="secondaryAction" dialogAction="cancel" onclick=self.link.callback(|_| Msg::Close) ><MatButton label="Cancel"/></span>
            </MatDialog>
        </div>
        }
    }
}
