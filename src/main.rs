use yew::prelude::*;
use material_yew::{Menu, MenuItem, Button, ButtonVariants};
#[function_component]
fn App() -> Html {
    html! {
        <div>
        <Button variant={ButtonVariants::Elevated}>
            {"Hello World"}
        </Button>
        <span style="position: relative">
            <md-filled-button id="btn-anchor">{"Set with idref"}</md-filled-button>
            <Menu open={true} anchor="btn-anchor">
                <MenuItem>
                    <div slot="headline">{"Apple"}</div>
                </MenuItem>
                <MenuItem>
                    <div slot="headline">{"Banana"}</div>
                </MenuItem>
            </Menu>
        </span>
        <span style="position: relative">
          // <md-filled-button id="usage-anchor">{"Set with idref"}</md-filled-button>
          // <md-menu ~open={true} id="usage-menu" anchor="usage-anchor">
          //   <md-menu-item>
          //     <div slot="headline">{"Apple"}</div>
          //   </md-menu-item>
          //   <md-menu-item>
          //     <div slot="headline">{"Banana"}</div>
          //   </md-menu-item>
          //   <md-menu-item>
          //     <div slot="headline">{"Cucumber"}</div>
          //   </md-menu-item>
          // </md-menu>
        </span>
        </div>
    }
}

fn main() {
    material_yew::load_core();
    yew::Renderer::<App>::new().render();
}