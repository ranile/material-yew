mod components;
pub mod macros;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_material_components::{
    MatDrawer, MatTopAppBarFixed, MatIconButton, MatButton, MatList, MatListItem,
};
use crate::components::{
    Home, Button, Components, Checkbox, Radio, Switch, Fab, IconButton, Icon,
    CircularProgress, Drawer, FormField, LinearProgress, List, IconButtonToggle, Slider,
    Tabs, Snackbar, Textfield, TextArea, Select,
};

use wasm_bindgen::prelude::*;
use std::cell::RefCell;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::Theme;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/components/button"]
    Button,
    #[to = "/components/checkbox"]
    Checkbox,
    #[to = "/components/radio"]
    Radio,
    #[to = "/components/switch"]
    Switch,
    #[to = "/components/fab"]
    Fab,
    #[to = "/components/icon-button-toggle"]
    IconButtonToggle,
    #[to = "/components/icon-button"]
    IconButton,
    #[to = "/components/icon"]
    Icon,
    #[to = "/components/circular-progress"]
    CircularProgress,
    #[to = "/components/drawer"]
    Drawer,
    #[to = "/components/form-field"]
    FormField,
    #[to = "/components/linear-progress"]
    LinearProgress,
    #[to = "/components/list"]
    List,
    #[to = "/components/slider"]
    Slider,
    #[to = "/components/tabs"]
    Tabs,
    #[to = "/components/snackbar"]
    Snackbar,
    #[to = "/components/textfield"]
    Textfield,
    #[to = "/components/textarea"]
    TextArea,
    #[to = "/components/select"]
    Select,
    #[to = "/components"]
    Components,
    #[to = "/"]
    Home,
}

type AppRouter = Router<AppRoute>;
type AppRouterAnchor = RouterAnchor<AppRoute>;

pub struct App {
    link: ComponentLink<Self>,
    /// `true` represents open; `false` represents close
    drawer_state: bool,
    // TODO tasks: Vec<FetchTask>,
}

pub enum Msg {
    NavIconClick,
    Opened,
    Closed,
}

pub struct SyntectData {
    theme: Option<Theme>,
    syntax_set: Option<SyntaxSet>,
}

thread_local!(pub static SYNTECT_DATA: RefCell<SyntectData> = RefCell::new(SyntectData {
    theme: None,
    syntax_set: None,
}));


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SYNTECT_DATA.with(|cell| {
            let mut data = cell.borrow_mut();
            data.theme = Some(syntect::dumps::from_binary(include_bytes!("../syntect-dumps/Material-Theme.theme")));
            data.syntax_set = Some(syntect::dumps::from_binary(include_bytes!("../syntect-dumps/rust.syntax")));
        });
        Self { link, drawer_state: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NavIconClick => {
                self.drawer_state = !self.drawer_state;
                true
            }
            Msg::Closed => {
                self.drawer_state = false;
                false
            }
            Msg::Opened => {
                self.drawer_state = true;
                false
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! { <>
        <MatDrawer open=self.drawer_state drawer_type="dismissible"
            onopened=self.link.callback(|_| Msg::Opened)
            onclosed=self.link.callback(|_| Msg::Closed)>

                <span class="drawer-title" slot="title">{"Components"}</span>

                <div class="drawer-content">
                    <MatList>
                        <AppRouterAnchor route=AppRoute::Button><MatListItem>{"Button"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Checkbox><MatListItem>{"Checkbox"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Radio><MatListItem>{"Radio"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Switch><MatListItem>{"Switch"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Fab><MatListItem>{"Floating Action Button"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::IconButton><MatListItem>{"Icon Button"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Icon><MatListItem>{"Icon"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::CircularProgress><MatListItem>{"Circular Progress"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::FormField><MatListItem>{"Form Field"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::LinearProgress><MatListItem>{"Linear Progress"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::List><MatListItem>{"List"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::IconButtonToggle><MatListItem>{"Icon Button Toggle"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Slider><MatListItem>{"Slider"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Tabs><MatListItem>{"Tabs"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Snackbar><MatListItem>{"Snackbar"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Textfield><MatListItem>{"Textfield"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::TextArea><MatListItem>{"TextArea"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Select><MatListItem>{"Select"}</MatListItem></AppRouterAnchor>
                    </MatList>
                </div>

                <div slot="appContent" class="app-content">
                    <MatTopAppBarFixed onnavigationiconclick=self.link.callback(|_| Msg::NavIconClick)>
                        <span slot="navigationIcon">
                            <MatIconButton icon="menu"></MatIconButton>
                        </span>
                        <div slot="title" class="app-title">
                            <AppRouterAnchor route=AppRoute::Home>{"Yew Material components"}</AppRouterAnchor>
                            <span class="action-item">
                                <AppRouterAnchor route=AppRoute::Components><MatButton label="Components"/></AppRouterAnchor>
                            </span>
                        </div>
                        <a slot="actionItems" class="action-item" href="https://github.com/hamza1311/material-yew-components"><MatButton label="GitHub"/></a>
                        <span slot="actionItems" class="action-item"><MatButton label="API Docs"/></span>
                    </MatTopAppBarFixed>
                    <main id="router-outlet">
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        // redirect=AppRouter::redirect(|route: Route| {
                        //     AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        // })
                    />
                    </main>
                </div>
            </MatDrawer>
        </> }
    }
}

impl App {
    fn switch(switch: AppRoute) -> Html {
        match switch {
            AppRoute::Home => html! { <Home /> },
            AppRoute::Components => html! { <Components /> },
            AppRoute::Button => html! { <Button /> },
            AppRoute::Checkbox => html! { <Checkbox /> },
            AppRoute::Radio => html! { <Radio /> },
            AppRoute::Switch => html! { <Switch /> },
            AppRoute::Fab => html! { <Fab /> },
            AppRoute::IconButton => html! { <IconButton /> },
            AppRoute::Icon => html! { <Icon /> },
            AppRoute::CircularProgress => html! { <CircularProgress /> },
            AppRoute::Drawer => html! { <Drawer /> },
            AppRoute::FormField => html! { <FormField /> },
            AppRoute::LinearProgress => html! { <LinearProgress /> },
            AppRoute::List => html! { <List /> },
            AppRoute::IconButtonToggle => html! { <IconButtonToggle /> },
            AppRoute::Slider => html! { <Slider /> },
            AppRoute::Tabs => html! { <Tabs /> },
            AppRoute::Snackbar => html! { <Snackbar /> },
            AppRoute::Textfield => html! { <Textfield /> },
            AppRoute::TextArea => html! { <TextArea /> },
            AppRoute::Select => html! { <Select /> },
        }
    }
}

fn html_to_element(html: &str) -> Html {
    let template: JsValue = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("template")
        .unwrap()
        .into();
    let template: web_sys::HtmlTemplateElement = template.into();
    let html = html.trim();
    template.set_inner_html(html);
    Html::VRef(template.content().first_child().unwrap().into())
}
