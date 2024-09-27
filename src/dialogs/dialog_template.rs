use dioxus::prelude::*;

use crate::dialogs::DialogState;
#[component]
pub fn DialogTemplate(
    dialog_state: Signal<DialogState>,
    header: String,
    modal_body: Element,
    ok_button: Element,
) -> Element {
    rsx! {
        div { id: "dialog-pad",
            div { class: "modal", style: "display: block;",
                div { class: "modal-dialog",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h5 { class: "model-title", {header} }
                            button {
                                r#type: "button",
                                class: "btn-close",
                                onclick: move |_| {
                                    dialog_state.set(DialogState::None);
                                }
                            }
                        }
                        div { class: "modal-body", {modal_body} }
                        div { class: "modal-footer",
                            div { class: "btn-group",
                                {ok_button},
                                button {
                                    class: "btn btn-outline-dark",
                                    onclick: move |_| { dialog_state.set(DialogState::None) },
                                    "Cancel"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
