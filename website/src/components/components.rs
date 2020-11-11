use crate::{AppRoute, AppRouterAnchor};
use yew::prelude::*;

pub struct Components {}

impl Component for Components {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let component_card = |name: &str, route| {
            let path = join("-", name.split(" ")).to_lowercase();

            html! {
                <AppRouterAnchor route=route>
                    <img src=format!("/assets/{}.png", path) />
                    <h6>{ name }</h6>
                </AppRouterAnchor>
            }
        };
        html! {
         <section class="components-grid">
             { component_card("Button", AppRoute::Button) }
             { component_card("Checkbox", AppRoute::Checkbox) }
             { component_card("Radio", AppRoute::Radio) }
             { component_card("Switch", AppRoute::Switch) }
             { component_card("Floating Action Button", AppRoute::Fab) }
             { component_card("Icon Button", AppRoute::IconButton) }
             { component_card("Icon", AppRoute::Icon) }
             { component_card("Circular Progress", AppRoute::CircularProgress) }
             { component_card("Dialog", AppRoute::Dialog) }
             // TODO { component_card("Drawer", AppRoute::Drawer) }
             { component_card("Formfield", AppRoute::FormField) }
             { component_card("Linear Progress", AppRoute::LinearProgress) }
             { component_card("List", AppRoute::List) }
             { component_card("Icon Button Toggle", AppRoute::IconButtonToggle) }
             { component_card("Slider", AppRoute::Slider) }
             { component_card("Tabs", AppRoute::Tabs) }
             { component_card("Snackbar", AppRoute::Snackbar) }
             { component_card("TextField", AppRoute::Textfield) }
             { component_card("TextArea", AppRoute::TextArea) }
             { component_card("Select", AppRoute::Select) }
             { component_card("Menu", AppRoute::Menu) }
         </section>
        }
    }
}

fn join<'a, T>(separator: &str, collection: T) -> String
where
    T: IntoIterator,
    T::Item: Into<&'a str>,
{
    let mut out = String::new();
    collection.into_iter().for_each(|it| {
        out.push_str(it.into());
        out.push_str(separator);
    });
    let out = out.strip_suffix("-");
    out.unwrap().to_owned()
}
