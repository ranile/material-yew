use yew::prelude::*;
use material_yew::{Button, ButtonVariants, Radio};
#[function_component]
fn App() -> Html {

    html! {
        <div>
        <Button variant={ButtonVariants::FilledTonal}>
        {"Click me"}
        </Button>
        <form>
            <label>
                <Radio name="animals" value="cats" checked=true />
                {"Birds"}
            </label>
        </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}