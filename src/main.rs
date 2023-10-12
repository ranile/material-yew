use yew::prelude::*;
use material_yew::{Menu, MenuItem, Button, ButtonVariants};
#[function_component]
fn App() -> Html {

    html! {
        <span style="position: relative">
            <Button variant={ButtonVariants::Elevated}>
                {"Hello World"}
            </Button>
            <Menu open={true}>
                <MenuItem>
                    <div slot="headline">{"Apple"}</div>
                </MenuItem>
                <MenuItem>
                    <div slot="headline">{"Banana"}</div>
                </MenuItem>
            </Menu>
        </span>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}