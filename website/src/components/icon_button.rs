use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::MatIconButton;
use yew::prelude::*;

pub struct IconButton {}

impl Component for IconButton {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let standard_icon_button = with_raw_code!(standard_icon_button { html! {
         <section class="demo">
             <MatIconButton icon="code"></MatIconButton>
         </section>
        }});
        let svg_icon_button = with_raw_code!(svg_icon_button { html! {
          <section class="demo">
             <MatIconButton>
                 <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"></path></svg>
             </MatIconButton>
         </section>
        }});
        let disabled_icon_button = with_raw_code!(disabled_icon_button { html! {
         <section class="demo">
             <MatIconButton disabled=true icon="code"></MatIconButton>
         </section>
        }});
        html! {<>
            <Codeblock title="Standard" code_and_html={standard_icon_button} />

            <Codeblock title="SVG" code_and_html={svg_icon_button} />

            <Codeblock title="Disabled" code_and_html={disabled_icon_button} />
        </>}
    }
}
