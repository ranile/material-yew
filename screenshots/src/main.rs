use yew::prelude::*;
use yew_router::prelude::*;
use yew_material::{
    MatButton, MatSelect, MatListItem, MatFormfield, select::{SelectedDetail, ListIndex},
};
use yew_router::agent::RouteRequest;

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/button"]
    Button,
    #[to = "/checkbox"]
    Checkbox,
    #[to = "/radio"]
    Radio,
    #[to = "/switch"]
    Switch,
    #[to = "/fab"]
    Fab,
    #[to = "/icon-button-toggle"]
    IconButtonToggle,
    #[to = "/icon-button"]
    IconButton,
    #[to = "/icon"]
    Icon,
    #[to = "/circular-progress"]
    CircularProgress,
    #[to = "/drawer"]
    Drawer,
    #[to = "/form-field"]
    FormField,
    #[to = "/linear-progress"]
    LinearProgress,
    #[to = "/list"]
    List,
    #[to = "/slider"]
    Slider,
    #[to = "/tabs"]
    Tabs,
    #[to = "/snackbar"]
    Snackbar,
    #[to = "/textfield"]
    Textfield,
    #[to = "/textarea"]
    TextArea,
    #[to = "/select"]
    Select,
    #[to = "/menu"]
    Menu,
    #[to = "/dialog"]
    Dialog,
}

impl ToString for AppRoute {
    fn to_string(&self) -> String {
        use AppRoute::*;

        match self {
            Button => "Button",
            Checkbox => "Checkbox",
            Radio => "Radio",
            Switch => "Switch",
            Fab => "Fab",
            IconButtonToggle => "Icon button toggle",
            IconButton => "Icon button",
            Icon => "Icon",
            CircularProgress => "Circular progress",
            Drawer => "Drawer",
            FormField => "Form field",
            LinearProgress => "Linear progress",
            List => "List",
            Slider => "Slider",
            Tabs => "Tabs",
            Snackbar => "Snackbar",
            Textfield => "Textfield",
            TextArea => "TextArea",
            Select => "Select",
            Menu => "Menu",
            Dialog => "Dialog",
        }.to_string()
    }
}

const COMPONENTS: [AppRoute; 21] = [
    AppRoute::Button,
    AppRoute::Checkbox,
    AppRoute::Radio,
    AppRoute::Switch,
    AppRoute::Fab,
    AppRoute::IconButtonToggle,
    AppRoute::IconButton,
    AppRoute::Icon,
    AppRoute::CircularProgress,
    AppRoute::Drawer,
    AppRoute::FormField,
    AppRoute::LinearProgress,
    AppRoute::List,
    AppRoute::Slider,
    AppRoute::Tabs,
    AppRoute::Snackbar,
    AppRoute::Textfield,
    AppRoute::TextArea,
    AppRoute::Select,
    AppRoute::Menu,
    AppRoute::Dialog,
];

type AppRouter = Router<AppRoute>;
type AppRouterAnchor = RouterAnchor<AppRoute>;

pub struct App {
    link: ComponentLink<Self>,
    router: RouteAgentDispatcher<()>,
}

pub enum Msg {
    Select(SelectedDetail),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, router: Default::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Select(details) => {
                let index = match details.index {
                    ListIndex::Single(Some(index)) => index,
                    _ => panic!("Unreachable executed"),
                };
                let component = COMPONENTS.get(index).expect("index too high. This should never happen").clone();
                let route = Route::from(component);
                self.router.send(RouteRequest::ChangeRoute(route));
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        use AppRoute::*;
        let on_selected = self.link.callback(|details| Msg::Select(details));
        let list_item = |comp: &AppRoute| {
            html! {
                <MatListItem value=format!("{:?}", comp) selected=true> { comp.to_string() } </MatListItem>
            }
        };

        html! { <>
        <main id="screenshots">
            <MatSelect label="Components" outlined=true onselected=on_selected >
                { for COMPONENTS.iter().map(list_item) }
            </MatSelect>
            <AppRouter render=AppRouter::render(Self::switch) />
        </main>
        </> }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Button => html! {
                <section id="button" class="grid">
                    <MatButton label="Button" />
                    <MatButton label="Button" outlined=true />
                    <MatButton label="Button" raised=true />
                    <MatButton label="Button" dense=true />
                </section>
            },
            AppRoute::Checkbox => html! {

            },
            AppRoute::Radio => html! {

            },
            AppRoute::Switch => html! {

            },
            AppRoute::Fab => html! {

            },
            AppRoute::IconButton => html! {

            },
            AppRoute::Icon => html! {

            },
            AppRoute::CircularProgress => html! {

            },
            AppRoute::Drawer => html! {

            },
            AppRoute::FormField => html! {

            },
            AppRoute::LinearProgress => html! {

            },
            AppRoute::List => html! {

            },
            AppRoute::IconButtonToggle => html! {

            },
            AppRoute::Slider => html! {

            },
            AppRoute::Tabs => html! {

            },
            AppRoute::Snackbar => html! {

            },
            AppRoute::Textfield => html! {

            },
            AppRoute::TextArea => html! {

            },
            AppRoute::Select => html! {

            },
            AppRoute::Menu => html! {

            },
            AppRoute::Dialog => html! {

            },
        }
    }
}

fn main() {
    yew::start_app::<App>()
}
