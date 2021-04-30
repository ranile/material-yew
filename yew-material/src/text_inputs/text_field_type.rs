use std::borrow::Cow;

/// The `TextFieldType` type
#[derive(Debug, Clone)]
pub enum TextFieldType {
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DatetimeLocal,
    Number,
    Color,
}

impl TextFieldType {
    pub fn to_cow_string(&self) -> Cow<'static, str> {
        use TextFieldType::*;
        let s = match self {
            Text => "text",
            Search => "search",
            Tel => "tel",
            Url => "url",
            Email => "email",
            Password => "password",
            Date => "date",
            Month => "month",
            Week => "week",
            Time => "time",
            DatetimeLocal => "datetime-local",
            Number => "number",
            Color => "color",
        };
        Cow::from(s)
    }
}
