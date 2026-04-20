use dioxus::prelude::*;

use crate::app::components::fetch_form::FetchForm;

#[derive(Props, Clone, PartialEq)]
pub struct AddUserDialogProps {
    pub open: bool,
    pub on_close: EventHandler<()>,
    pub on_user_added: EventHandler<String>,
}

#[component]
pub fn AddUserDialog(props: AddUserDialogProps) -> Element {
    if !props.open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed inset-0 bg-black/60 flex items-center justify-center z-50",
            onclick: move |_| props.on_close.call(()),
            div {
                class: "bg-bg-panel border border-border-gold rounded p-6 w-[36rem] max-w-[90vw] flex flex-col gap-4",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex items-center justify-between",
                    h2 { class: "text-lg text-star-5", "添加用户" }
                    button {
                        class: "text-fg-dim hover:text-fg-base text-sm",
                        onclick: move |_| props.on_close.call(()),
                        "关闭"
                    }
                }

                FetchForm {
                    on_success: move |pid: String| {
                        props.on_user_added.call(pid);
                        props.on_close.call(());
                    },
                }
            }
        }
    }
}
