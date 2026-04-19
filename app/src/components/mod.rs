use dioxus::prelude::*;

pub mod fetch_form;
pub mod labels;
pub mod welcome;

#[component]
pub fn Root() -> Element {
    rsx! {
        div { class: "min-h-screen bg-bg-base text-fg-base p-6",
            h1 { class: "text-2xl text-star-5", "鸣潮抽卡记录" }
            p { class: "text-fg-dim mt-2", "组件树接入中（Task 14-24）" }
        }
    }
}
