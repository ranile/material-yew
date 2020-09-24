mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use yew_material_components::{MatDrawer, MatTopAppBarFixed, MatIconButton, MatButton};
use crate::components::{Home, Button, Components, Checkbox, Radio, Switch, Fab, IconButton, Icon, CircularProgress, Drawer, FormField, LinearProgress};

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
    #[to = "/components"]
    Components,
    #[to = "/"]
    Home,
}

type AppRouter = Router<AppRoute>;
type AppRouterAnchor = RouterAnchor<AppRoute>;
type AppRouteButton = Router<AppRoute>;

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

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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

                <span slot="title">{"Components"}</span>

                <div style="padding: 1em 16px 0 16px;display: flex;flex-direction: column;gap: 1em;">
                // TODO MatList
                    <span>{"Drawer content 1"}</span>
                    <span>{"Drawer content 2"}</span>
                    <span>{"Drawer content 3"}</span>
                    <span>{"Drawer content 4"}</span>
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
        }
    }
}
