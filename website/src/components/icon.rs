use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::MatIcon;
use yew::prelude::*;

pub struct Icon {}

impl Component for Icon {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let icons = with_raw_code!(icons { html! {
         <section class="demo">
             <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
             <MatIcon>{"sentiment_dissatisfied"}</MatIcon>
             <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
             <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
             <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
         </section>
        }});
        html! {<>
            <Codeblock title="Icons" code_and_html={icons} />
        </>}
    }
}
