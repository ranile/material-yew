//! A Material components library for Yew. It wrpas around Material Web Components exposing Yew components
//!
//! Example usage:
//! ```rust
//! use yew_material_components::MatButton;
//! use yew::html;
//!
//! html! {
//!     <MatButton label="Click me!" />
//! }
//! ```
//!
//! More information can be found in the [GitHub README](https://github.com/hamza1311/yew-material-components)

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use yew::NodeRef;

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
    ($comp: ident, $props: ty, $html: expr, $mwc_to_initialize: ident) => {
        pub struct $comp {
            props: $props
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

fn add_event_listener(node_ref: &NodeRef, name: &str, func: impl Fn() + 'static, closure_to_store_in: &mut Option<Closure<dyn FnMut()>>) {
    let element = node_ref.cast::<yew::web_sys::Element>().unwrap();
    *closure_to_store_in = Some(Closure::wrap(Box::new(func) as Box<dyn FnMut()>));
    element.add_event_listener_with_callback(name, closure_to_store_in.as_ref().unwrap().as_ref().unchecked_ref());
}

fn read_boolean_property(element: &yew::web_sys::Element, name: &str) -> bool {
    js_sys::Reflect::get(&element, &JsValue::from_str(name))
        .expect("property is not found")
        .as_bool()
        .expect("property is not a bool")
}

fn set_element_property(element: &yew::web_sys::Element, key: &str, value: &JsValue) -> bool {
    js_sys::Reflect::set(&element, &JsValue::from_str(key), value)
        .expect("Setting property failed")
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
pub use list::{MatList, MatListItem, MatCheckListItem, MatRadioListItem};

pub mod icon_button_toggle;
pub use icon_button_toggle::MatIconButtonToggle;
