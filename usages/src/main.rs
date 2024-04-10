#![allow(dead_code)]
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};
use crate::route::Route;

mod usage;
mod route;

fn switch(route: Route) -> Html {
    match route {
        Route::Menu => usage::test_menu(html!{}),
        Route::Button => usage::test_button(html! { "Button" }),
        Route::CircularProgress => usage::test_circular_progress(),
        Route::IconButton => todo!("icon"),
        Route::Radio => usage::test_radio(),
        Route::ListItem => html!(),
        Route::List => usage::test_list(usage::test_list_item(html! { "List Item" })),
        Route::Switch => usage::test_switch(),
        Route::Chip => usage::test_chip(html!()),
        Route::SubMenu => todo!(),
        Route::Fab => usage::test_fab(html!("icon")),
        Route::LinearProgress => usage::test_linear_progress(),
        Route::Slider => usage::test_slider(),
        Route::Checkbox => usage::test_checkbox(),
        Route::MenuItem => html!(),
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
        <Switch<Route> render={switch} />
        </BrowserRouter>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
