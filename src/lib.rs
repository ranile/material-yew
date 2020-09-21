use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
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
            link: yew::ComponentLink<Self>,
            props: $props
        }

        impl yew::Component for $comp {
            type Message = ();
            type Properties = Props;

            fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
                $mwc_to_initialize::ensure_loaded();
                Self { props, link }
            }

            fn update(&mut self, _msg: Self::Message) -> ShouldRender {
                false
            }

            fn change(&mut self, props: Self::Properties) -> bool {
                self.props = props;
                true
            }

            fn view(&self) -> Html {
                $html(self)
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

pub fn add_event_listener(node_ref: &NodeRef, name: &str, func: impl Fn() + 'static, closure_to_store_in: &mut Option<Closure<dyn FnMut()>>) {
    let element = node_ref.cast::<yew::web_sys::Element>().unwrap();
    *closure_to_store_in = Some(Closure::wrap(Box::new(func) as Box<dyn FnMut()>));
    element.add_event_listener_with_callback(name, closure_to_store_in.as_ref().unwrap().as_ref().unchecked_ref());
}

mod button;
pub use button::MatButton;

mod circular_progress;
pub use circular_progress::MatCircularProgress;

mod checkbox;
pub use checkbox::MatCheckbox;

mod circular_progress_four_color;
pub use circular_progress_four_color::MatCircularProgressFourColor;

mod drawer;
pub use drawer::MatDrawer;

mod top_app_bar;
pub use top_app_bar::MatTopAppBar;

mod icon_button;
pub use icon_button::MatIconButton;

mod fab;
pub use fab::MatFab;

mod form_field;
pub use form_field::MatFormfield;
