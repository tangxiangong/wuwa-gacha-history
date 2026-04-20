use crate::core::{EnrichedPull, VersionGroup, group_by_version};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SummaryViewProps {
    pub pulls: Vec<EnrichedPull>,
}

#[component]
pub fn SummaryView(props: SummaryViewProps) -> Element {
    let groups = group_by_version(&props.pulls);
    if groups.is_empty() {
        return rsx! { div { class: "text-fg-dim", "暂无数据" } };
    }
    rsx! {
        div { class: "flex flex-col gap-3",
            for g in groups.into_iter() {
                VersionRow { g }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct VersionRowProps {
    g: VersionGroup,
}

#[component]
fn VersionRow(props: VersionRowProps) -> Element {
    let g = &props.g;
    let up_names = g.up_names.join("、");
    let start_str = g
        .start
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "—".into());
    let version_label = format!("v{}", g.version);
    let total_str = g.pulls.len().to_string();
    let r5r4_str = format!("{} / {}", g.r5.len(), g.r4.len());
    let up_stray_str = format!("{} 歪 {}", g.ups, g.stray);

    rsx! {
        div { class: "bg-bg-panel border border-border-gold/30 rounded p-3 grid grid-cols-6 gap-3 items-center text-sm",
            div { class: "text-star-5 text-lg", "{version_label}" }
            div { class: "text-fg-dim font-mono text-xs", "{start_str}" }
            Cell { v: total_str,       k: "总抽数".to_string() }
            Cell { v: r5r4_str,        k: "5★ / 4★".to_string() }
            Cell { v: up_stray_str,    k: "UP / 歪".to_string() }
            div { class: "col-span-1 truncate text-fg-base", title: "{up_names}", "{up_names}" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct CellProps {
    v: String,
    k: String,
}

#[component]
fn Cell(props: CellProps) -> Element {
    rsx! {
        div { class: "flex flex-col",
            span { class: "text-star-5 text-base", "{props.v}" }
            span { class: "text-fg-dim text-xs", "{props.k}" }
        }
    }
}
