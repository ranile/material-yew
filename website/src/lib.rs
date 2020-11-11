mod components;
pub mod macros;

use crate::components::{
    Button, Checkbox, CircularProgress, Components, Dialog, Drawer, Fab, FormField, Home, Icon,
    IconButton, IconButtonToggle, LinearProgress, List, Menu, Radio, Select, Slider, Snackbar,
    Switch, Tabs, TextArea, Textfield,
};
use yew::prelude::*;
use yew_material::{
    drawer::{MatDrawerAppContent, MatDrawerTitle},
    top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle},
    MatButton, MatDrawer, MatIconButton, MatList, MatListItem, MatTopAppBarFixed,
};
use yew_router::prelude::*;

use std::cell::RefCell;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;
use wasm_bindgen::prelude::*;

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
    #[to = "/components/menu"]
    Menu,
    #[to = "/components/dialog"]
    Dialog,
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
}

pub enum Msg {
    NavIconClick,
    Opened,
    Closed,
}

pub struct SyntectData {
    pub theme: Option<Theme>,
    pub syntax_set: Option<SyntaxSet>,
}

thread_local!(pub static SYNTECT_DATA: RefCell<SyntectData> = RefCell::new(SyntectData {
    theme: None,
    syntax_set: None,
}));

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            drawer_state: false,
        }
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
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! { <>
        <MatDrawer open=self.drawer_state drawer_type="dismissible"
            onopened=self.link.callback(|_| Msg::Opened)
            onclosed=self.link.callback(|_| Msg::Closed)>

                <MatDrawerTitle>
                    <span class="drawer-title">{"Components"}</span>
                </MatDrawerTitle>

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
                        <AppRouterAnchor route=AppRoute::Menu><MatListItem>{"Menu"}</MatListItem></AppRouterAnchor>
                        <AppRouterAnchor route=AppRoute::Dialog><MatListItem>{"Dialog"}</MatListItem></AppRouterAnchor>
                    </MatList>
                </div>
                <MatDrawerAppContent>
                    <div class="app-content">
                        <MatTopAppBarFixed onnavigationiconclick=self.link.callback(|_| Msg::NavIconClick)>
                            <MatTopAppBarNavigationIcon>
                                <MatIconButton icon="menu"></MatIconButton>
                            </MatTopAppBarNavigationIcon>

                            <MatTopAppBarTitle>
                                <div class="app-title">
                                    <AppRouterAnchor route=AppRoute::Home>{"Yew Material"}</AppRouterAnchor>
                                    <span class="action-item">
                                        <AppRouterAnchor route=AppRoute::Components><MatButton label="Components"/></AppRouterAnchor>
                                    </span>
                                </div>
                            </MatTopAppBarTitle>

                            <MatTopAppBarActionItems>
                                <a class="action-item" href="https://github.com/hamza1311/material-yew-components"><MatButton label="GitHub"/></a>
                            </MatTopAppBarActionItems>

                            <MatTopAppBarActionItems>
                                <span class="action-item"><MatButton label="API Docs"/></span>
                            </MatTopAppBarActionItems>

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
                </MatDrawerAppContent>
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
            AppRoute::Menu => html! { <Menu /> },
            AppRoute::Dialog => html! { <Dialog /> },
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
