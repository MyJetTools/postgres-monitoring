use std::rc::Rc;

use crate::dialogs::DialogTemplate;
use dioxus::prelude::*;

use super::DialogState;

#[component]
pub fn RenderConfirmation(text: Rc<String>, on_ok: EventHandler<()>) -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();
    rsx! {
        DialogTemplate {
            dialog_state,
            header: "Confirmation",
            modal_body: rsx! {
                div { {text.as_str()} }
            },
            ok_button: rsx! {
                button {
                    class: "btn btn-danger",
                    onclick: move |_| {
                        on_ok.call(());
                        dialog_state.set(DialogState::None);
                    },
                    "Confirm"
                }
            }
        }
    }
}

/*

    let phrase = format!(
        "Are you sure you want to delete the ignore event for {:?} for application {} with marker{} ?",
        model.level,
        model.application,
        model.marker
    );

 let itm_to_request = model.as_ref().clone();
                        let env = env.clone();
                        spawn(async move {
                            delete_ignore_event(env.to_string(), itm_to_request).await.unwrap();
                            main_state.write().reset_data();
                            dialog_state.set(DialogState::None)
                        });
*/
