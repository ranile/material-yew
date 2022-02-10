use crate::{AppLink, AppRoute};
use yew::prelude::*;

pub struct Components {}

impl Component for Components {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        let component_card = |name: &str, route| {
            let path = join("-", name.split(' ')).to_lowercase();

            html! {
                 <AppLink to={route}>
                     <img src={format!("/assets/{}.png", path)} />
                     <h6>{name}</h6>
                 </AppLink>
            }
        };
        html! {
          <section class="components-grid">
              {component_card("Button", AppRoute::Button)}
              {component_card("Checkbox", AppRoute::Checkbox)}
              {component_card("Radio", AppRoute::Radio)}
              {component_card("Switch", AppRoute::Switch)}
              {component_card("Floating Action Button", AppRoute::Fab)}
              {component_card("Icon Button", AppRoute::IconButton)}
              {component_card("Icon", AppRoute::Icon)}
              {component_card("Circular Progress", AppRoute::CircularProgress)}
              {component_card("Dialog", AppRoute::Dialog)}
              // TODO { component_card("Drawer", AppRoute::Drawer)}
              {component_card("Formfield", AppRoute::FormField)}
              {component_card("Linear Progress", AppRoute::LinearProgress)}
              {component_card("List", AppRoute::List)}
              {component_card("Icon Button Toggle", AppRoute::IconButtonToggle)}
              {component_card("Slider", AppRoute::Slider)}
              {component_card("Tabs", AppRoute::Tabs)}
              {component_card("Snackbar", AppRoute::Snackbar)}
              {component_card("TextField", AppRoute::Textfield)}
              {component_card("TextArea", AppRoute::TextArea)}
              {component_card("Select", AppRoute::Select)}
              {component_card("Menu", AppRoute::Menu)}
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
