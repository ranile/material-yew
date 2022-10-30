use std::fmt;

/// Equivalent to typescript type
/// `'avatar'|'icon'|'medium'|'large'|'control'|null`
///
/// See `GraphicType` [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-item-1)
#[derive(Clone, PartialEq, Debug)]
pub enum GraphicType {
    Avatar,
    Icon,
    Medium,
    Large,
    Control,
    Null,
}

impl GraphicType {
    pub fn as_str(&self) -> &'static str {
        use GraphicType::*;
        match self {
            Avatar => "avatar",
            Icon => "icon",
            Medium => "medium",
            Large => "large",
            Control => "control",
            Null => "null",
        }
    }
}

impl fmt::Display for GraphicType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
