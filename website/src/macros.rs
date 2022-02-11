pub fn read_until_close(s: &str) -> &str {
    let mut depth = 1;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            _ => {}
        }

        if depth == 0 {
            return &s[..i];
        }
    }

    s
}

pub fn highlight(code: &str, extension: &str) -> String {
    crate::SYNTECT_DATA.with(|cell| {
        let data = cell.borrow();
        let syntax = data
            .syntax_set
            .as_ref()
            .unwrap()
            .find_syntax_by_extension(extension)
            .unwrap();
        syntect::html::highlighted_html_for_string(
            code,
            data.syntax_set.as_ref().unwrap(),
            syntax,
            data.theme.as_ref().unwrap(),
        )
    })
}

#[macro_export]
macro_rules! with_raw_code {
    ($key:ident { $expr:expr }) => {{
        const CODE: &str = include_str!(concat!("../../../", file!()));

        let marker_start = CODE
            .find(concat!("with_raw_code!(", stringify!($key)))
            .expect(&format!("marker {:?} not found", stringify!($key)));
        let body_offset = CODE[marker_start..].find('{').unwrap();
        let code = crate::macros::read_until_close(&CODE[marker_start + body_offset + 9..]);
        let code = unindent::unindent(code);

        let html_string = crate::macros::highlight(&code, "rs");

        (html_string, $expr)
    }};
}
