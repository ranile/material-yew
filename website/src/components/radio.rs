use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::MatRadio;
use yew::prelude::*;

pub struct Radio {}

impl Component for Radio {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let standard = with_raw_code!(standard { html! {
         <section class="demo">
             <MatRadio name="some name" />
         </section>
        }});
        let checked = with_raw_code!(checked { html! {
         <section class="demo">
             <MatRadio checked=true />
         </section>
        }});
        let disabled = with_raw_code!(disabled { html! {
         <section class="demo">
             <MatRadio disabled=true />
         </section>
        }});

        html! {<>
            <Codeblock title="Standard Radio" code_and_html={standard} />

            <Codeblock title="Checked Radio" code_and_html={checked} />

            <Codeblock title="Disabled Radio" code_and_html={disabled} />
        </>}
    }
}
