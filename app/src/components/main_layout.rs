use dioxus::prelude::*;

use crate::components::sidebar::Sidebar;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "flex h-screen bg-bg-base",
            Sidebar {
                on_add_user: move |_| { /* Task 23 will flip GlobalState::add_user_open */ },
                on_export:   move |_| { /* Task 24 will flip GlobalState::export_open */ },
            }
            main { class: "flex-1 overflow-auto p-6",
                div { class: "text-fg-dim", "ContentArea 待接入（Task 19）" }
            }
        }
    }
}
