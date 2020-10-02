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

#[macro_export]
macro_rules! with_raw_code {
    ($key:ident { $expr:expr }) => {{
        const CODE: &str = include_str!(concat!("../../", file!()));

        let marker_start = CODE
            .find(concat!("with_raw_code!(", stringify!($key)))
            .expect(&format!("marker {:?} not found", stringify!($key)));
        let body_offset = CODE[marker_start..].find('{').unwrap();
        let code = crate::macros::read_until_close(&CODE[marker_start + body_offset + 9..]);
        let code = unindent::unindent(code);
        (code, $expr)
    }};
}
