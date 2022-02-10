use std::fmt;
use yew::prelude::*;

/// Dialog action type.
#[derive(Clone, PartialEq)]
pub enum ActionType {
    /// Binds `to slot` of `primaryAction`
    Primary,
    /// Binds `to slot` of `secondaryAction`
    Secondary,
}

impl ActionType {
    fn as_str(&self) -> &'static str {
        match self {
            ActionType::Primary => "primaryAction",
            ActionType::Secondary => "secondaryAction",
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Props for [`MatDialogAction`]
#[derive(Properties, PartialEq, Clone)]
pub struct ActionProps {
    pub action_type: ActionType,
    #[prop_or_default]
    pub action: Option<String>,
    pub children: Children,
}

/// Defines actions for [`MatDialog`][crate::MatDialog].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatDialogAction;

impl Component for MatDialogAction {
    type Message = ();
    type Properties = ActionProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let children = props.children.iter().map(|child| {
            match child {
                Html::VTag(mut vtag) => {
                    vtag.add_attribute("slot", props.action_type.to_string());
                    if let Some(action) = props.action.as_ref() {
                        vtag.add_attribute("dialogAction", action.to_owned());
                   }
                    Html::VTag(vtag)
               }
                _ => html! {
                    <span slot={props.action_type.to_string()} dialogAction={props.action.clone()}>
                        {child}
                    </span>
               }
           }
       }).collect::<Html>();

        html! {
             {children}
        }
    }
}
