use dioxus::prelude::*;

#[component]
pub fn MainLayout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-bg-base text-fg-base p-6",
            h2 { class: "text-xl text-star-5", "主界面" }
            p { class: "text-fg-dim mt-2",
                "Sidebar / ContentArea 接入中（Task 15 + Task 19）"
            }
        }
    }
}
