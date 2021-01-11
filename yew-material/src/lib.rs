#![doc(html_root_url = "/docs")]

//! A Material components library for [Yew](https://yew.rs). It wrpas around [Material Web Components](https://github.com/material-components/material-components-web-components) exposing Yew components.
//!
//! Example usage:
//! ```rust
//! use yew::html;
//! use yew_material::MatButton;
//!
//! html! {
//!     <MatButton label="Click me!" />
//! }
//! ```
//!
//! All the main components from the modules are re-exported.
//! The specialized components used for populating slots and models can be
//! accessed from their respective modules.
//!
//! More information can be found on the [website](https://yew-material.web.app) and in the [GitHub README](https://github.com/hamza1311/yew-material)

use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CustomEvent, Event};
mod utils;

// this macro is defined here so we can access it in the modules
macro_rules! loader_hack {
    ($ty:ty) => {
        #[allow(dead_code)]
        static LOADED: std::sync::Once = std::sync::Once::new();
        impl $ty {
            #[allow(dead_code)]
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
        paste::paste! {
            #[doc = "The `mwc-" $mwc_name "` component"]
            #[doc = ""]
            #[doc = "[MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/"$mwc_name")"]
            pub struct $comp {
                props: $props
            }
        }
        impl yew::Component for $comp {
            type Message = ();
            type Properties = $props;

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
        false => None,
    }
}

fn to_option_string(s: &str) -> Option<&str> {
    match s {
        "" => None,
        _ => Some(s),
    }
}

fn event_into_details(event: &Event) -> JsValue {
    JsValue::from(event)
        .dyn_into::<CustomEvent>()
        .unwrap_or_else(|_| panic!("could not convert to CustomEvent"))
        .detail()
}
fn event_details_into<T: JsCast>(event: &Event) -> T {
    event_into_details(event).unchecked_into::<T>()
}

#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "button")]
#[doc(hidden)]
pub use button::MatButton;

#[cfg(feature = "circular-progress")]
pub mod circular_progress;
#[cfg(feature = "circular-progress")]
#[doc(hidden)]
pub use circular_progress::MatCircularProgress;

#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "checkbox")]
#[doc(hidden)]
pub use checkbox::MatCheckbox;

#[cfg(feature = "circular-progress-four-color")]
pub mod circular_progress_four_color;
#[cfg(feature = "circular-progress-four-color")]
#[doc(hidden)]
pub use circular_progress_four_color::MatCircularProgressFourColor;

#[cfg(feature = "drawer")]
pub mod drawer;
#[cfg(feature = "drawer")]
#[doc(hidden)]
pub use drawer::MatDrawer;

#[cfg(feature = "top-app-bar")]
pub mod top_app_bar;
#[cfg(feature = "top-app-bar")]
#[doc(hidden)]
pub use top_app_bar::MatTopAppBar;

#[cfg(feature = "icon-button")]
pub mod icon_button;
#[cfg(feature = "icon-button")]
#[doc(hidden)]
pub use icon_button::MatIconButton;

#[cfg(feature = "fab")]
pub mod fab;
#[cfg(feature = "fab")]
#[doc(hidden)]
pub use fab::MatFab;

#[cfg(feature = "formfield")]
pub mod form_field;
#[cfg(feature = "formfield")]
#[doc(hidden)]
pub use form_field::MatFormfield;

#[cfg(feature = "icon")]
pub mod icon;
#[cfg(feature = "icon")]
#[doc(hidden)]
pub use icon::MatIcon;

#[cfg(feature = "linear-progress")]
pub mod linear_progress;
#[cfg(feature = "linear-progress")]
#[doc(hidden)]
pub use linear_progress::MatLinearProgress;

#[cfg(feature = "radio")]
pub mod radio;
#[cfg(feature = "radio")]
#[doc(hidden)]
pub use radio::MatRadio;

#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "switch")]
#[doc(hidden)]
pub use switch::MatSwitch;

#[cfg(feature = "top-app-bar-fixed")]
pub mod top_app_bar_fixed;
#[cfg(feature = "top-app-bar-fixed")]
#[doc(hidden)]
pub use top_app_bar_fixed::MatTopAppBarFixed;

#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dialog")]
#[doc(hidden)]
pub use dialog::MatDialog;

#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "list")]
#[doc(no_inline)]
#[doc(hidden)]
pub use list::{MatCheckListItem, MatList, MatListItem, MatRadioListItem};

#[cfg(feature = "icon-button-toggle")]
pub mod icon_button_toggle;
#[cfg(feature = "icon-button-toggle")]
#[doc(hidden)]
pub use icon_button_toggle::MatIconButtonToggle;

#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "slider")]
#[doc(hidden)]
pub use slider::MatSlider;

#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "tabs")]
#[doc(no_inline)]
#[doc(hidden)]
pub use tabs::{MatTab, MatTabBar};

#[cfg(feature = "snackbar")]
pub mod snackbar;
#[cfg(feature = "snackbar")]
#[doc(hidden)]
pub use snackbar::MatSnackbar;

#[cfg(any(feature = "textfield", feature = "textarea"))]
pub mod text_inputs;
#[cfg(feature = "textarea")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::MatTextArea;
#[cfg(feature = "textfield")]
#[doc(no_inline)]
#[doc(hidden)]
pub use text_inputs::MatTextField;

#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "select")]
#[doc(hidden)]
pub use select::MatSelect;

#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menu")]
#[doc(hidden)]
pub use menu::MatMenu;

#[doc(hidden)]
pub use utils::WeakComponentLink;

#[wasm_bindgen(module = "/../build/core.js")]
extern "C" {
    #[derive(Debug)]
    type Ripple;

    #[wasm_bindgen(getter, static_method_of = Ripple)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(Ripple);
