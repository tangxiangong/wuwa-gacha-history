use dioxus::prelude::*;
use wuwa_gacha_history::EnrichedPull;

#[derive(Props, Clone, PartialEq)]
pub struct SummaryViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn SummaryView(props: SummaryViewProps) -> Element {
    let _ = &props.pulls;
    rsx! { div { class: "text-fg-dim", "版本总结视图（Task 22）" } }
}
