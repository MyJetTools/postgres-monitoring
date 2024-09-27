use std::rc::Rc;

use dioxus::prelude::EventHandler;

#[derive(Clone)]
pub enum DialogState {
    None,

    Confirmation {
        text: Rc<String>,
        on_ok: EventHandler<()>,
    },
}
