use yew::prelude::*;
use mat_web_comp::{MatDrawer, MatTopAppBar, MatIconButton};

pub struct Drawer {
    link: ComponentLink<Self>,
    state: bool,
}

pub enum Msg {
    Click,
    Opened,
    Closed
}

impl Component for Drawer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Drawer { link, state: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.state = true;
                true
            },
            Msg::Closed => {
                self.state = false;
                false
            }
            Msg::Opened => {
                self.state = true;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {

        html! {
            <MatDrawer open=self.state drawer_type="modal"
            onopened=self.link.callback(|_| Msg::Opened)
            onclosed=self.link.callback(|_| Msg::Closed)>

                <span slot="title">{"Drawer Title"}</span>
                <span slot="subtitle">{"subtitle"}</span>
                <div style="padding: 0px 16px 0 16px;">
                    <p>{"Drawer content!"}</p>

                    <MatIconButton>
                          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                    </MatIconButton>

                    <MatIconButton icon="gavel"/>
                </div>
                <div slot="appContent">
                    <MatTopAppBar center_title=true onnavigationiconclick=self.link.callback(|_| Msg::Click)>
                        <span slot="navigationIcon">
                            <MatIconButton icon="menu"></MatIconButton>
                        </span>
                        <div slot="title">{"Title"}</div>
                        <span slot="actionItems"><MatIconButton icon="file_download"></MatIconButton></span>
                    </MatTopAppBar>
                    <div>
                        <p>{"Main Content!"}</p>
                    </div>
                </div>
            </MatDrawer>
        }
    }
}
