use gloo::events::EventListener;
use std::borrow::Cow;
use web_sys::{Event, EventTarget};
use yew::Callback;

pub struct Listener<IN> {
    el: Option<EventListener>,
    cb: Option<Callback<IN>>,
}

impl<IN> Listener<IN> {
    pub fn set<S, F, N>(
        &mut self,
        target: &EventTarget,
        event_type: S,
        cb: &yew::Callback<IN>,
        new_cb: N,
    ) where
        S: Into<Cow<'static, str>>,
        F: FnMut(&Event) + 'static,
        N: FnOnce(Callback<IN>) -> F,
    {
        if match self.cb.as_ref() {
            None => true,
            Some(old_cb) => old_cb != cb,
        } {
            self.el = Some(EventListener::new(
                target,
                event_type,
                (new_cb)(cb.to_owned()),
            ));
            self.cb = Some(cb.to_owned());
        }
    }
}

impl<IN> Default for Listener<IN> {
    fn default() -> Self {
        Self { el: None, cb: None }
    }
}
