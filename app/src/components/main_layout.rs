use dioxus::prelude::*;

use crate::components::{GlobalState, content_area::ContentArea, sidebar::Sidebar};

#[component]
pub fn MainLayout() -> Element {
    let mut state = use_context::<GlobalState>();
    rsx! {
        div { class: "flex h-screen bg-bg-base",
            Sidebar {
                on_add_user: move |_| state.add_user_open.set(true),
                on_export:   move |_| state.export_open.set(true),
            }
            main { class: "flex-1 overflow-auto p-6",
                ContentArea {}
            }
        }
    }
}
