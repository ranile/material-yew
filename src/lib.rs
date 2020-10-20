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
//! More information can be found on the [website](https://yew-material.web.app) and in the [GitHub README](https://github.com/hamza1311/yew-material)

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use yew::{NodeRef};
use web_sys::Element;
use wasm_bindgen::prelude::*;
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

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "button")]
pub use button::MatButton;

#[cfg(feature = "circular-progress")]
pub mod circular_progress;
#[cfg(feature = "circular-progress")]
pub use circular_progress::MatCircularProgress;

#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "checkbox")]
pub use checkbox::MatCheckbox;

#[cfg(feature = "circular-progress-four-color")]
pub mod circular_progress_four_color;
#[cfg(feature = "circular-progress-four-color")]
pub use circular_progress_four_color::MatCircularProgressFourColor;

#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "drawer")]
pub use drawer::MatDrawer;

#[cfg(feature = "top-app-bar")]
pub mod top_app_bar;
#[cfg(feature = "top-app-bar")]
pub use top_app_bar::MatTopAppBar;

#[cfg(feature = "icon-button")]
pub mod icon_button;
#[cfg(feature = "icon-button")]
pub use icon_button::MatIconButton;

#[cfg(feature = "fab")]
pub mod fab;
#[cfg(feature = "fab")]
pub use fab::MatFab;

#[cfg(feature = "formfield")]
pub mod form_field;
#[cfg(feature = "formfield")]
pub use form_field::MatFormfield;

#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "icon")]
pub use icon::MatIcon;

#[cfg(feature = "linear-progress")]
pub mod linear_progress;
#[cfg(feature = "linear-progress")]
pub use linear_progress::MatLinearProgress;

#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "radio")]
pub use radio::MatRadio;

#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
pub use switch::MatSwitch;

#[cfg(feature = "top-app-bar-fixed")]
pub mod top_app_bar_fixed;
#[cfg(feature = "top-app-bar-fixed")]
pub use top_app_bar_fixed::MatTopAppBarFixed;

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dialog")]
pub use dialog::MatDialog;

#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "list")]
pub use list::*;

#[cfg(feature = "icon-button-toggle")]
pub mod icon_button_toggle;
#[cfg(feature = "icon-button-toggle")]
pub use icon_button_toggle::MatIconButtonToggle;

#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "slider")]
pub use slider::MatSlider;

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
pub use tabs::{MatTab, MatTabBar};

#[cfg(feature = "snackbar")]
pub mod snackbar;
#[cfg(feature = "snackbar")]
pub use snackbar::MatSnackbar;

pub mod text_inputs;
pub use text_inputs::*;

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "select")]
pub use select::MatSelect;

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menu")]
pub use menu::MatMenu;

pub use utils::WeakComponentLink;

#[wasm_bindgen(module = "/build/core.js")]
extern "C" {
    #[derive(Debug)]
    type Ripple;

    #[wasm_bindgen(getter, static_method_of = Ripple)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Ripple);
