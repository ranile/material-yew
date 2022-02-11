use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::MatSwitch;
use yew::prelude::*;

pub struct Switch {}

impl Component for Switch {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let standard = with_raw_code!(standard { html! {
         <section class="demo">
             <h3>{"Switch"}</h3>
             <MatSwitch />
         </section>
        }});

        let checked = with_raw_code!(checked { html! {
         <section class="demo">
             <h3>{"Switch"}</h3>
             <MatSwitch checked=true />
         </section>
        }});

        let disabled = with_raw_code!(disabled { html! {
         <section class="demo">
             <h3>{"Switch"}</h3>
             <MatSwitch disabled=true />
         </section>
        }});

        let disabled_checked = with_raw_code!(disabled_checked { html! {
         <section class="demo">
             <h3>{"Switch"}</h3>
             <MatSwitch disabled=true checked=true />
         </section>
        }});

        html! {<>
            <Codeblock title="Standard" code_and_html={standard} />

            <Codeblock title="Checked" code_and_html={checked} />

            <Codeblock title="Disabled" code_and_html={disabled} />

            <Codeblock title="Disabled checked" code_and_html={disabled_checked} />
        </>}
    }
}
