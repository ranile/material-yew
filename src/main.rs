use yew::prelude::*;
use material_yew::{Menu, MenuItem, Button, ButtonVariants, List, ListItem};
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
        <div>
            <List>
                <ListItem>{"Apple"}</ListItem>
                <ListItem>{"Banana"}</ListItem>
                <ListItem>
                    <div slot="headline">
                        {"Cucumber"}
                    </div>
                    <div slot="supporting-text">
                        {"Cucumbers are long green fruits that are just as long as this multi-line description"}
                    </div>
                </ListItem>
            </List>
        </div>
        </div>
    }
}

fn main() {
    material_yew::load_core();
    yew::Renderer::<App>::new().render();
}