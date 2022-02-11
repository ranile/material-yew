use website::{App, SYNTECT_DATA};

fn main() {
    SYNTECT_DATA.with(|cell| {
        let mut data = cell.borrow_mut();
        data.theme = Some(syntect::dumps::from_binary(include_bytes!(
            "../syntect-dumps/Material-Theme-Lighter.theme"
        )));
        data.syntax_set = Some(syntect::dumps::from_binary(include_bytes!(
            "../syntect-dumps/syntax-set.syntax"
        )));
    });
    yew::start_app::<App>();
}
