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

fn to_option(value: bool) -> Option<&'static str> {
    match value {
        true => Some("true"),
        false => None
    }
}

mod button;
pub use button::*;

mod circular_progress;
pub use circular_progress::*;
