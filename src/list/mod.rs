pub mod list;
pub use list::*;

pub mod list_item;
pub use list_item::*;

pub mod check_list_item;
pub use check_list_item::MatCheckListItem;

pub mod radio_list_item;
mod list_index;
mod selected_detail;

pub use list_index::ListIndex;
pub use selected_detail::{SelectedDetail, IndexDiff};
pub use radio_list_item::MatRadioListItem;

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

