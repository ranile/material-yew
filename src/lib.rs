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

mod button;
pub use button::*;
