use material_yew::{
    dialog::{ActionType, MatDialogAction},
    icon_button_toggle::{MatOffIconButtonToggle, MatOnIconButtonToggle},
    select::{ListIndex, SelectedDetail},
    text_inputs::TextFieldType,
    MatButton, MatCheckbox, MatCircularProgress, MatDialog, MatFab, MatFormfield, MatIcon,
    MatIconButton, MatIconButtonToggle, MatLinearProgress, MatList, MatListItem, MatMenu, MatRadio,
    MatSelect, MatSlider, MatSnackbar, MatSwitch, MatTab, MatTabBar, MatTextArea, MatTextField,
    WeakComponentLink,
};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/button")]
    Button,
    #[at("/checkbox")]
    Checkbox,
    #[at("/radio")]
    Radio,
    #[at("/switch")]
    Switch,
    #[at("/fab")]
    Fab,
    #[at("/icon-button-toggle")]
    IconButtonToggle,
    #[at("/icon-button")]
    IconButton,
    #[at("/icon")]
    Icon,
    #[at("/circular-progress")]
    CircularProgress,
    #[at("/form-field")]
    FormField,
    #[at("/linear-progress")]
    LinearProgress,
    #[at("/list")]
    List,
    #[at("/slider")]
    Slider,
    #[at("/tabs")]
    Tabs,
    #[at("/snackbar")]
    Snackbar,
    #[at("/textfield")]
    Textfield,
    #[at("/textarea")]
    TextArea,
    #[at("/select")]
    Select,
    #[at("/menu")]
    Menu,
    #[at("/dialog")]
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
        }
        .to_string()
    }
}

const COMPONENTS: [AppRoute; 20] = [
    AppRoute::Button,
    AppRoute::Checkbox,
    AppRoute::Radio,
    AppRoute::Switch,
    AppRoute::Fab,
    AppRoute::IconButtonToggle,
    AppRoute::IconButton,
    AppRoute::Icon,
    AppRoute::CircularProgress,
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

pub struct App {
    menu_link: WeakComponentLink<MatMenu>,
}

pub enum Msg {
    Select(SelectedDetail),
    OpenMenu,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            menu_link: Default::default(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(details) => {
                let index = match details.index {
                    ListIndex::Single(Some(index)) => index,
                    _ => panic!("Unreachable executed"),
                };
                let _component = *COMPONENTS
                    .get(index)
                    .expect("index too high. This should never happen");
                
// FIXME                use_history().unwrap().push(component);
                true
            }
            Msg::OpenMenu => {
                self.menu_link.show();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_selected = ctx.link().callback(Msg::Select);
        let list_item = |comp: &AppRoute| {
            html! {
                 <MatListItem value={format!("{:?}", comp)}> {comp.to_string()} </MatListItem>
            }
        };

        let menu = |_| -> Html { html! { } };
        /* FIXME
        // this is a special case as its stateful
        let menu =  |_| -> Html { html! { <>
            <span onclick={ctx.link().callback(|_| Msg::OpenMenu)} >
                <MatButton label="Show meat" />
            </span>
            <div>
                <MatMenu  menu_link={self.menu_link} open=true>
                    <MatListItem>{"Chicken"}</MatListItem>
                    <MatListItem>{"Mutton"}</MatListItem>
                    <MatListItem>{"Beef"}</MatListItem>
                </MatMenu>
            </div>
        </>}};
        */

        html! { <>
        <BrowserRouter>
            <main id="screenshots">
                <MatSelect label="Components" outlined=true onselected={on_selected} >
                    { for COMPONENTS.iter().map(list_item)}
                </MatSelect>
                <Switch<AppRoute> render={menu} />
            </main>
        </BrowserRouter>
        </>}
    }
}

impl App {
    fn switch(switch: AppRoute, menu: Html) -> Html {
        let ret = match switch {
            AppRoute::Button => {
                html! {
                     <section id="button" class="grid">
                         <MatButton label="Button" />
                         <MatButton label="Button" outlined=true />
                         <MatButton label="Button" raised=true />
                         <MatButton label="Button" dense=true outlined=true />
                     </section>
                }
            }
            AppRoute::Checkbox => {
                html! {
                     <section id="checkbox" class="grid">
                         <MatCheckbox />
                         <MatCheckbox checked=true />
                     </section>
                }
            }
            AppRoute::Radio => {
                html! {
                     <section id="radio" class="grid">
                         <MatRadio />
                         <MatRadio checked=true />
                     </section>
                }
            }
            AppRoute::Switch => {
                html! {
                     <section id="switch" class="grid">
                         <MatSwitch />
                         <MatSwitch checked=true />
                     </section>
                }
            }
            AppRoute::Fab => {
                html! {
                     <section id="fab" class="grid">
                         <div>
                             <MatFab icon="edit" />
                             <MatFab icon="add" mini=true />
                         </div>
                         <MatFab icon="shopping_cart" label="Add to cart" extended=true />
                     </section>
                }
            }
            AppRoute::IconButton => {
                html! {
                     <section id="icon-button" class="grid">
                         <MatIconButton icon="backup" />
                         <MatIconButton icon="code" />
                         <MatIconButton icon="cast" />
                         <MatIconButton icon="favorite" />
                     </section>
                }
            }
            AppRoute::Icon => {
                html! {
                     <section id="icon" class="grid">
                         <MatIcon>{"backup"}</MatIcon>
                         <MatIcon>{"code"}</MatIcon>
                         <MatIcon>{"cast"}</MatIcon>
                         <MatIcon>{"favorite"}</MatIcon>
                     </section>
                }
            }
            AppRoute::CircularProgress => {
                html! {
                     <section id="circular-progress" class="grid">
                         <MatCircularProgress progress=0.1 />
                         <MatCircularProgress progress=0.2 />
                         <MatCircularProgress progress=0.4 />
                         <MatCircularProgress progress=0.6 />
                         <MatCircularProgress progress=0.8 />
                         <MatCircularProgress progress=1.0 />
                     </section>
                }
            }
            AppRoute::FormField => {
                html! {
                     <section id="form-field" class="container">
                         <MatFormfield align_end=true>
                             <MatTextField label="Password" field_type={TextFieldType::Password} required=true />
                         </MatFormfield>
                     </section>
                }
            }
            AppRoute::LinearProgress => {
                html! {
                     <section id="linear-progress" class="grid">
                         <MatLinearProgress progress=0.75 buffer=0.5 />
                         <MatLinearProgress indeterminate=true />
                     </section>
                }
            }
            AppRoute::List => {
                html! {
                     <section id="list" class="container">
                         <MatList>
                             <MatListItem>{"Item 0"}</MatListItem>
                             <MatListItem selected=true>{"Item 1"}</MatListItem>
                             <MatListItem>{"Item 2"}</MatListItem>
                         </MatList>
                     </section>
                }
            }
            AppRoute::IconButtonToggle => {
                html! {
                     <section id="icon-button-toggle" class="grid">
                         <MatIconButtonToggle on_icon="sentiment_very_satisfied" off_icon="sentiment_very_dissatisfied" />

                         <MatIconButtonToggle>
                             <MatOnIconButtonToggle>
                                 <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                             </MatOnIconButtonToggle>

                             <MatOffIconButtonToggle>
                                 <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0V0zm0 0h24v24H0V0z"/><path d="M16.59 7.58L10 14.17l-3.59-3.58L5 12l5 5 8-8zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z"/></svg>
                             </MatOffIconButtonToggle>
                         </MatIconButtonToggle>
                     </section>
                }
            }
            AppRoute::Slider => {
                html! {
                     <section id="slider" class="grid">
                         <MatSlider />
                     </section>
                }
            }
            AppRoute::Tabs => {
                html! {
                 <section id="tabs" class="container">
                     <MatTabBar>
                         <MatTab label="one" />
                         <MatTab label="two" />
                         <MatTab label="three" />
                     </MatTabBar>
                 </section>
                }
            }
            AppRoute::Snackbar => {
                html! {
                     <MatSnackbar label_text="Can't send photo. Retry in 5 seconds." timeout_ms={-1} open=true />
                }
            }
            AppRoute::Textfield => {
                html! {
                     <section id="textfield" class="grid">
                         <MatTextField label="Name" />
                         <MatTextField label="Email" field_type={TextFieldType::Email} outlined=true />
                     </section>
                }
            }
            AppRoute::TextArea => {
                html! {
                     <section id="textarea" class="grid">
                         <MatTextArea label="Thoughts?" />
                         <MatTextArea label="Outlined Thoughts" outlined=true />
                     </section>
                }
            }
            AppRoute::Select => {
                html! {
                     <section id="select" class="container">
                         <MatSelect label="Components" outlined=true>
                             <MatListItem value="2"> {"Vegetables"} </MatListItem>
                             <MatListItem value="3"> {"Meat"} </MatListItem>
                         </MatSelect>
                     </section>
                }
            }
            AppRoute::Menu => {
                html! {
                     <section id="menu" class="container">
                         {menu}
                     </section>
                }
            }
            AppRoute::Dialog => {
                html! {
                     <MatDialog open=true>
                         {"Delete item?"}
                         <MatDialogAction action_type={ActionType::Primary} action={AttrValue::from("ok")}>
                             <MatButton label="Yes" />
                         </MatDialogAction>

                         <MatDialogAction action_type={ActionType::Secondary} action={AttrValue::from("cancel")}>
                             <MatButton label="No" />
                         </MatDialogAction>
                     </MatDialog>
                }
            }
        };
        ret
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
