use yew::prelude::*;
use material_yew::{Button, ButtonVariants};
#[function_component]
fn App() -> Html {

    html! {
        <Button variant={ButtonVariants::Filled}>
        {"Click me"}
        </Button>
        // <form>
        //     <label>
        //         <Radio name="animals" value="cats" checked=true />
        //         {"Birds"}
        //     </label>
        // </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}