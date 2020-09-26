use yew::prelude::*;
use yew_material_components::{MatSnackbar, MatButton, MatIconButton};

pub struct Snackbar {
    link: ComponentLink<Self>,
    default_open: bool,
    leading_open: bool,
    stacked_open: bool,
}

pub enum Msg {
    OpenDefault,
    OpenLeading,
    OpenStacked,
    DefaultClosed,
    LeadingClosed,
    StackedClosed,
}

impl Component for Snackbar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { default_open: false, leading_open: false, link, stacked_open: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OpenDefault => { self.default_open = true; }
            Msg::OpenLeading => { self.leading_open = true; }
            Msg::OpenStacked => { self.stacked_open = true; }
            Msg::DefaultClosed => { self.default_open = false; }
            Msg::LeadingClosed => { self.leading_open = false; }
            Msg::StackedClosed => { self.stacked_open = false; }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
            <section style="margin: 1em 0;">
                <span onclick=self.link.callback(|_| Msg::OpenDefault)>
                    <MatButton label="Default" raised=true  />
                 </span>
                <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." open=self.default_open
                    onclosed=self.link.callback(|_| Msg::DefaultClosed)>
                    <span slot="action">
                        <MatButton label="RETRY" />
                    </span>

                    <span class="snackbar-dismiss-slot" slot="dismiss">
                        <MatIconButton icon="close" />
                    </span>
                </MatSnackbar>
            </section>

            <section style="margin: 1em 0;">
                <span onclick=self.link.callback(|_| Msg::OpenLeading)>
                    <MatButton label="Leading" raised=true  />
                 </span>
                <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." open=self.leading_open leading=true
                    onclosed=self.link.callback(|_| Msg::LeadingClosed)>
                    <span slot="action">
                        <MatButton label="RETRY" />
                    </span>

                    <span class="snackbar-dismiss-slot" slot="dismiss">
                        <MatIconButton icon="close" />
                    </span>
                </MatSnackbar>
            </section>

            <section style="margin: 1em 0;">
                <span onclick=self.link.callback(|_| Msg::OpenStacked)>
                    <MatButton label="Default" raised=true  />
                 </span>
                <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." open=self.stacked_open stacked=true
                    onclosed=self.link.callback(|_| Msg::StackedClosed)>
                    <span slot="action">
                        <MatButton label="RETRY" />
                    </span>

                    <span class="snackbar-dismiss-slot" slot="dismiss">
                        <MatIconButton icon="close" />
                    </span>
                </MatSnackbar>
            </section>
        </>}
    }
}
