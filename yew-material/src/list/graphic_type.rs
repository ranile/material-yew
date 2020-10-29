/// Equivalent to typescript type `'avatar'|'icon'|'medium'|'large'|'control'|null`
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

impl ToString for GraphicType {
    fn to_string(&self) -> String {
        use GraphicType::*;
        match self {
            Avatar => "avatar",
            Icon => "icon",
            Medium => "medium",
            Large => "large",
            Control => "control",
            Null => "null",
        }.to_string()
    }
}

