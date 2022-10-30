/// The `Corner` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#propertiesattributes)
#[derive(Clone, PartialEq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopStart,
    TopEnd,
    BottomStart,
    BottomEnd,
}

impl ToString for Corner {
    fn to_string(&self) -> String {
        use Corner::*;
        match self {
            TopLeft => "TOP_LEFT",
            TopRight => "TOP_RIGHT",
            BottomLeft => "BOTTOM_LEFT",
            BottomRight => "BOTTOM_RIGHT",
            TopStart => "TOP_START",
            TopEnd => "TOP_END ",
            BottomStart => "BOTTOM_START",
            BottomEnd => "BOTTOM_END",
        }
        .to_string()
    }
}

/// The `MenuCorner` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#propertiesattributes)
#[derive(Clone, PartialEq)]
pub enum MenuCorner {
    Start,
    End,
}

impl ToString for MenuCorner {
    fn to_string(&self) -> String {
        use MenuCorner::*;
        match self {
            Start => "START",
            End => "END",
        }
        .to_string()
    }
}

/// The `DefaultFocusState` type
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#propertiesattributes)
#[derive(Clone, PartialEq)]
pub enum DefaultFocusState {
    None,
    ListRoot,
    FirstItem,
    LastItem,
}

impl ToString for DefaultFocusState {
    fn to_string(&self) -> String {
        use DefaultFocusState::*;
        match self {
            None => "NONE",
            ListRoot => "LIST_ROOT",
            FirstItem => "FIRST_ITEM",
            LastItem => "LAST_ITEM",
        }
        .to_string()
    }
}
