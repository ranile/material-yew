use std::borrow::Cow;
use std::fmt;

/// Equivalent to typescript type
/// `'avatar'|'icon'|'medium'|'large'|'control'|null`
///
/// See `GraphicType` [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-item-1)
#[derive(Clone, Debug)]
pub enum GraphicType {
    Avatar,
    Icon,
    Medium,
    Large,
    Control,
    Null,
}

impl GraphicType {
    pub fn to_cow_string(&self) -> Cow<'static, str> {
        use GraphicType::*;
        let s = match self {
            Avatar => "avatar",
            Icon => "icon",
            Medium => "medium",
            Large => "large",
            Control => "control",
            Null => "null",
        };
        Cow::from(s)
    }
}

impl fmt::Display for GraphicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_cow_string())
    }
}
