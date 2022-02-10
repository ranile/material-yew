mod components;
pub mod macros;

use crate::components::{
    Button, Checkbox, CircularProgress, Components, Dialog, Drawer, Fab, FormField, Home, Icon,
    IconButton, IconButtonToggle, LinearProgress, List, Menu, Radio, Select, Slider, Snackbar,
    Switch, Tabs, TextArea, Textfield,
};
use material_yew::{
    drawer::{MatDrawerAppContent, MatDrawerTitle},
    top_app_bar_fixed::{MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle},
    MatButton, MatDrawer, MatIconButton, MatList, MatListItem, MatTopAppBarFixed,
};
use yew::prelude::*;
use yew_router::prelude::*;

use std::cell::RefCell;
use syntect::highlighting::Theme;
use syntect::parsing::SyntaxSet;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/components/button")]
    Button,
    #[at("/components/checkbox")]
    Checkbox,
    #[at("/components/radio")]
    Radio,
    #[at("/components/switch")]
    Switch,
    #[at("/components/fab")]
    Fab,
    #[at("/components/icon-button-toggle")]
    IconButtonToggle,
    #[at("/components/icon-button")]
    IconButton,
    #[at("/components/icon")]
    Icon,
    #[at("/components/circular-progress")]
    CircularProgress,
    #[at("/components/drawer")]
    Drawer,
    #[at("/components/form-field")]
    FormField,
    #[at("/components/linear-progress")]
    LinearProgress,
    #[at("/components/list")]
    List,
    #[at("/components/slider")]
    Slider,
    #[at("/components/tabs")]
    Tabs,
    #[at("/components/snackbar")]
    Snackbar,
    #[at("/components/textfield")]
    Textfield,
    #[at("/components/textarea")]
    TextArea,
    #[at("/components/select")]
    Select,
    #[at("/components/menu")]
    Menu,
    #[at("/components/dialog")]
    Dialog,
    #[at("/components")]
    Components,
    #[at("/")]
    Home,
}

type AppLink = Link<AppRoute>;

pub struct App {
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

    fn create(_: &Context<Self>) -> Self {
        Self {
            drawer_state: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let is_on_mobile = is_on_mobile();

        let components = if !is_on_mobile {
            html! { <MatButton label="Components"/>}
        } else {
            html! {
                 <MatIconButton label="Components">
                     <img src="/assets/components.png" alt="Components" />
                 </MatIconButton>
            }
        };

        let docs = if !is_on_mobile {
            html! { <MatButton label="API Docs"/>}
        } else {
            html! { <MatIconButton icon="description" label="API Docs" />}
        };

        let github = if !is_on_mobile {
            html! { <MatButton label="GitHub" />}
        } else {
            html! {
                 <MatIconButton label="GitHub">
                     <img src="/assets/github.png" alt="GitHub logo" />
                 </MatIconButton>
            }
        };

        html! { <>
        <MatDrawer open={self.drawer_state} drawer_type="dismissible"
            onopened={link.callback(|_| Msg::Opened)}
            onclosed={link.callback(|_| Msg::Closed)}>

                <MatDrawerTitle>
                    <span class="drawer-title">{"Components"}</span>
                </MatDrawerTitle>

                <div class="drawer-content">
                    <MatList>
                        <AppLink to={AppRoute::Button}><MatListItem>{"Button"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Checkbox}><MatListItem>{"Checkbox"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Radio}><MatListItem>{"Radio"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Switch}><MatListItem>{"Switch"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Fab}><MatListItem>{"Floating Action Button"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::IconButton}><MatListItem>{"Icon Button"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Icon}><MatListItem>{"Icon"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::CircularProgress}><MatListItem>{"Circular Progress"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::FormField}><MatListItem>{"Form Field"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::LinearProgress}><MatListItem>{"Linear Progress"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::List}><MatListItem>{"List"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::IconButtonToggle}><MatListItem>{"Icon Button Toggle"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Slider}><MatListItem>{"Slider"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Tabs}><MatListItem>{"Tabs"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Snackbar}><MatListItem>{"Snackbar"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Textfield}><MatListItem>{"Textfield"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::TextArea}><MatListItem>{"TextArea"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Select}><MatListItem>{"Select"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Menu}><MatListItem>{"Menu"}</MatListItem></AppLink>
                        <AppLink to={AppRoute::Dialog}><MatListItem>{"Dialog"}</MatListItem></AppLink>
                    </MatList>
                </div>
                <MatDrawerAppContent>
                    <div class="app-content">
                        <MatTopAppBarFixed onnavigationiconclick={link.callback(|_| Msg::NavIconClick)}>
                            <MatTopAppBarNavigationIcon>
                                <MatIconButton icon="menu"></MatIconButton>
                            </MatTopAppBarNavigationIcon>

                            <MatTopAppBarTitle>
                                <div class="app-title">
                                    <AppLink to={AppRoute::Home}>
                                        <h1>{"Material Yew"}</h1>
                                    </AppLink>
                                    <span class="action-item">
                                        <AppLink to={AppRoute::Components}>
                                            {components}
                                        </AppLink>
                                    </span>
                                </div>
                            </MatTopAppBarTitle>

                            <MatTopAppBarActionItems>
                                <a class="action-item" href="https://github.com/hamza1311/yew-material">
                                    {github}
                                </a>
                            </MatTopAppBarActionItems>

                            <MatTopAppBarActionItems>
                                <a class="action-item" href="/docs/material_yew">
                                    {docs}
                                </a>
                            </MatTopAppBarActionItems>

                        </MatTopAppBarFixed>
                        <main id="router-outlet">
                        <BrowserRouter>
                            <yew_router::Switch<AppRoute> render={yew_router::Switch::render(Self::switch)} />
                        </BrowserRouter>
                        </main>
                    </div>
                </MatDrawerAppContent>
            </MatDrawer>
        </>}
    }
}

impl App {
    fn switch(switch: &AppRoute) -> Html {
        match switch {
            AppRoute::Home => html! { <Home />},
            AppRoute::Components => html! { <Components />},
            AppRoute::Button => html! { <Button />},
            AppRoute::Checkbox => html! { <Checkbox />},
            AppRoute::Radio => html! { <Radio />},
            AppRoute::Switch => html! { <Switch />},
            AppRoute::Fab => html! { <Fab />},
            AppRoute::IconButton => html! { <IconButton />},
            AppRoute::Icon => html! { <Icon />},
            AppRoute::CircularProgress => html! { <CircularProgress />},
            AppRoute::Drawer => html! { <Drawer />},
            AppRoute::FormField => html! { <FormField />},
            AppRoute::LinearProgress => html! { <LinearProgress />},
            AppRoute::List => html! { <List />},
            AppRoute::IconButtonToggle => html! { <IconButtonToggle />},
            AppRoute::Slider => html! { <Slider />},
            AppRoute::Tabs => html! { <Tabs />},
            AppRoute::Snackbar => html! { <Snackbar />},
            AppRoute::Textfield => html! { <Textfield />},
            AppRoute::TextArea => html! { <TextArea />},
            AppRoute::Select => html! { <Select />},
            AppRoute::Menu => html! { <Menu />},
            AppRoute::Dialog => html! { <Dialog />},
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
    Html::VRef(template.content().first_child().unwrap())
}

pub fn is_on_mobile() -> bool {
    gloo_utils::window()
        .match_media("(max-width: 600px)")
        .unwrap()
        .unwrap()
        .matches()
}
