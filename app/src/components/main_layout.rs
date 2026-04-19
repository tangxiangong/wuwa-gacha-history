use dioxus::prelude::*;

use crate::components::{content_area::ContentArea, sidebar::Sidebar};

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "flex h-screen bg-bg-base",
            Sidebar {
                on_add_user: move |_| { /* Task 23 */ },
                on_export:   move |_| { /* Task 24 */ },
            }
            main { class: "flex-1 overflow-auto p-6",
                ContentArea {}
            }
        }
    }
}
