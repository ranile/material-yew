use yew_router::prelude::*;
#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/menu")]
    Menu,
    #[at("/button")]
    Button,
    #[at("/circularprogress")]
    CircularProgress,
    #[at("/iconbutton")]
    IconButton,
    #[at("/radio")]
    Radio,
    #[at("/listitem")]
    ListItem,
    #[at("/list")]
    List,
    #[at("/switch")]
    Switch,
    #[at("/chip")]
    Chip,
    #[at("/submenu")]
    SubMenu,
    #[at("/fab")]
    Fab,
    #[at("/linearprogress")]
    LinearProgress,
    #[at("/slider")]
    Slider,
    #[at("/checkbox")]
    Checkbox,
    #[at("/menuitem")]
    MenuItem,
}
