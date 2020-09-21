use yew::prelude::*;
use mat_web_comp::{MatSwitch, MatButton, MatFormfield};

pub struct Switch {
    link: ComponentLink<Self>,
    change: bool,
    node_ref: NodeRef
}

pub enum Msg {
    Change(bool),
}

impl Component for Switch {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, change: false, node_ref: NodeRef::default() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Change(val) => {
                self.change = val;
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; width: max-content; gap: 1em; padding: 0 1em;">
            {&self.change.to_string()}
                <MatSwitch onchange=self.link.callback(|val| Msg::Change(val)) ref=self.node_ref.clone() checked=self.change />
                <MatSwitch disabled=true />
                <br/>
                <MatFormfield label="Test" align_end=true>
                    <MatSwitch checked=true />
                </MatFormfield>
            </div>
        }
    }
}
