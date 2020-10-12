//! A Material components library for [Yew](https://yew.rs). It wrpas around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components
//!
//! Example usage:
//! ```rust
//! use yew_material::MatButton;
//! use yew::html;
//!
//! html! {
//!     <MatButton label="Click me!" />
//! }
//! ```
//!
//! More information can be found on the [website]() and in the [GitHub README](https://github.com/hamza1311/yew-material)

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use yew::{NodeRef};
use web_sys::Element;
mod utils;

// this macro is defined here so we can access it in the modules
macro_rules! loader_hack {
    ($ty:ty) => {
        static LOADED: std::sync::Once = std::sync::Once::new();
        impl $ty {
            fn ensure_loaded() {
                LOADED.call_once(|| {
                    <$ty>::_dummy_loader();
                });
            }
        }
    };
}

macro_rules! component {
    ($comp: ident, $props: ty, $html: expr, $mwc_to_initialize: ident, $mwc_name: literal) => {
    use paste::paste;
        paste! {
            #[doc = "The `mwc-" $mwc_name "` component"]
            #[doc = ""]
            #[doc = "[MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/"$mwc_name")"]
            pub struct $comp {
                props: $props
            }
        }
        impl yew::Component for $comp {
            type Message = ();
            type Properties = Props;

            fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
                $mwc_to_initialize::ensure_loaded();
                Self { props }
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, props: Self::Properties) -> bool {
                self.props = props;
                true
            }

            fn view(&self) -> Html {
                $html(&self.props)
            }
        }
    };
}

fn to_option(value: bool) -> Option<&'static str> {
    match value {
        true => Some("true"),
        false => None
    }
}


fn to_option_string(s: &str) -> Option<&str> {
    match s {
        "" => None,
        _ => Some(s)
    }
}


fn add_event_listener(node_ref: &NodeRef, name: &str, func: impl Fn() + 'static, closure_to_store_in: &mut Option<Closure<dyn FnMut()>>) {
    let element = node_ref.cast::<Element>().unwrap();
    *closure_to_store_in = Some(Closure::wrap(Box::new(func) as Box<dyn FnMut()>));
    element.add_event_listener_with_callback(name, closure_to_store_in.as_ref().unwrap().as_ref().unchecked_ref())
        .expect(&format!("Failed to add listener to event {}", name))
}

fn add_event_listener_with_one_param(node_ref: &NodeRef, name: &str, func: impl Fn(JsValue) + 'static, closure_to_store_in: &mut Option<Closure<dyn FnMut(JsValue)>>) {
    let element = node_ref.cast::<Element>().unwrap();
    *closure_to_store_in = Some(Closure::wrap(Box::new(func) as Box<dyn FnMut(JsValue)>));
    element.add_event_listener_with_callback(name, closure_to_store_in.as_ref().unwrap().as_ref().unchecked_ref())
        .expect(&format!("Failed to add listener to event {}", name))
}


pub mod button;
pub use button::MatButton;

pub mod circular_progress;
pub use circular_progress::MatCircularProgress;

pub mod checkbox;
pub use checkbox::MatCheckbox;

pub mod circular_progress_four_color;
pub use circular_progress_four_color::MatCircularProgressFourColor;

pub mod drawer;
pub use drawer::MatDrawer;

pub mod top_app_bar;
pub use top_app_bar::MatTopAppBar;

pub mod icon_button;
pub use icon_button::MatIconButton;

pub mod fab;
pub use fab::MatFab;

pub mod form_field;
pub use form_field::MatFormfield;

pub mod icon;
pub use icon::MatIcon;

pub mod linear_progress;
pub use linear_progress::MatLinearProgress;

pub mod radio;
pub use radio::MatRadio;

pub mod switch;
pub use switch::MatSwitch;

pub mod top_app_bar_fixed;
pub use top_app_bar_fixed::MatTopAppBarFixed;

pub mod dialog;
pub use dialog::MatDialog;

pub mod list;
pub use list::*;

pub mod icon_button_toggle;
pub use icon_button_toggle::MatIconButtonToggle;

pub mod slider;
pub use slider::MatSlider;

pub mod tabs;
pub use tabs::{MatTab, MatTabBar};

pub mod snackbar;
pub use snackbar::MatSnackbar;

pub mod text_inputs;
pub use text_inputs::*;

pub mod select;
pub use select::MatSelect;

pub mod menu;
pub use menu::MatMenu;

pub use utils::WeakComponentLink;
