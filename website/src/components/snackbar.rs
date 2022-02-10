use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::{MatButton, MatIconButton, MatSnackbar, WeakComponentLink};
use yew::prelude::*;

pub struct Snackbar {
    default_link: WeakComponentLink<MatSnackbar>,
    leading_link: WeakComponentLink<MatSnackbar>,
    stacked_link: WeakComponentLink<MatSnackbar>,
}

pub enum Msg {
    OpenDefault,
    OpenLeading,
    OpenStacked,
    DefaultClosed(Option<String>),
    LeadingClosed(Option<String>),
    StackedClosed(Option<String>),
}

impl Component for Snackbar {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            default_link: WeakComponentLink::default(),
            leading_link: WeakComponentLink::default(),
            stacked_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenDefault => {
                self.default_link.show();
            }
            Msg::OpenLeading => {
                self.leading_link.show();
            }
            Msg::OpenStacked => {
                self.stacked_link.show();
            }
            Msg::DefaultClosed(reason) => {
                gloo_console::log!(&format!("default closed with reason {:?}", reason))
            }
            Msg::LeadingClosed(reason) => {
                gloo_console::log!(&format!("leading closed with reason {:?}", reason))
            }
            Msg::StackedClosed(reason) => {
                gloo_console::log!(&format!("stacked closed with reason {:?}", reason))
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let default = with_raw_code!(default { html! {
         <section style="margin: 1em 0;">
             <span onclick={link.callback(|_| Msg::OpenDefault)}>
                 <MatButton label="Open default snackbar" raised=true  />
              </span>
             <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." snackbar_link={self.default_link.clone()}
                 onclosed={link.callback(Msg::DefaultClosed)}>
                 <span slot="action">
                     <MatButton label="RETRY" />
                 </span>

                 <span class="snackbar-dismiss-slot" slot="dismiss">
                     <MatIconButton icon="close" />
                 </span>
             </MatSnackbar>
         </section>
        }});

        let leading = with_raw_code!(leading { html! {
         <section style="margin: 1em 0;">
             <span onclick={link.callback(|_| Msg::OpenLeading)}>
                 <MatButton label="Open leading snackbar" raised=true  />
              </span>
             <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." snackbar_link={self.leading_link.clone()} leading=true
                 onclosed={link.callback(Msg::LeadingClosed)}>
                 <span slot="action">
                     <MatButton label="RETRY" />
                 </span>

                 <span class="snackbar-dismiss-slot" slot="dismiss">
                     <MatIconButton icon="close" />
                 </span>
             </MatSnackbar>
         </section>
        }});

        let stacked = with_raw_code!(stacked { html! {
         <section style="margin: 1em 0;">
             <span onclick={link.callback(|_| Msg::OpenStacked)}>
                 <MatButton label="Open stacked snackbar" raised=true  />
              </span>
             <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." snackbar_link={self.stacked_link.clone()} stacked=true
                 onclosed={link.callback(Msg::StackedClosed)}>
                 <span slot="action">
                     <MatButton label="RETRY" />
                 </span>

                 <span class="snackbar-dismiss-slot" slot="dismiss">
                     <MatIconButton icon="close" />
                 </span>
             </MatSnackbar>
         </section>
        }});

        html! {<>
            <Codeblock title="Default" code_and_html={default} />

            <Codeblock title="Leading" code_and_html={leading} />

            <Codeblock title="Stacked" code_and_html={stacked} />
        </>}
    }
}
