use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::{MatCheckbox, MatFormfield, MatRadio, MatSwitch};
use yew::prelude::*;

pub struct FormField {}

impl Component for FormField {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let checkbox = with_raw_code!(checkbox { html! {
        <section>
            <div class="demo">
                <h3>{"Standard"}</h3>
                <MatFormfield label="This is a checkbox">
                    <MatCheckbox />
                </MatFormfield>
            </div>

            <div class="demo">
                <h3>{"Align End"}</h3>
                <MatFormfield label="This is another checkbox" align_end=true>
                    <MatCheckbox />
                </MatFormfield>
            </div>
        </section>}});

        let switch = with_raw_code!(switch { html! {
         <section class="demo">
             <MatFormfield label="This is a switch">
                 <MatSwitch />
             </MatFormfield>
         </section>
        }});

        let radio = with_raw_code!(radio { html! {
         <section class="demo">
             <MatFormfield label="This is a radio button">
                 <MatRadio name="radio-name" />
             </MatFormfield>
         </section>
        }});

        html! {<>
            <Codeblock title="Checkbox" code_and_html={checkbox} />

            <Codeblock title="Switch" code_and_html={switch} />

            <Codeblock title="Radio" code_and_html={radio} />
        </>}
    }
}
