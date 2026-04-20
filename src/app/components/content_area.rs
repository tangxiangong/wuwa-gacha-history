use crate::{
    app::{
        api,
        components::{
            self, GlobalState, filter_panel::FilterPanel, labels::card_pool_label,
            pagination::Pagination, record_table::RecordTable,
        },
    },
    core::{EnrichedPull, GachaFilter, GachaRecord, QualityLevel, banner_stats, enrich_pulls},
};
use dioxus::prelude::*;

const PAGE_SIZE: usize = 20;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Bars,
    Cards,
    Summary,
}

#[component]
pub fn ContentArea() -> Element {
    let state = use_context::<GlobalState>();

    let player_id_opt = state.player_id.read().clone();
    let active_pool_opt = *state.active_pool.read();

    let Some(player_id) = player_id_opt else {
        return rsx! { div { class: "text-fg-dim", "请选择一个用户" } };
    };
    let Some(active_pool) = active_pool_opt else {
        return rsx! { div { class: "text-fg-dim", "请选择一个卡池" } };
    };

    let quality = use_signal::<Option<QualityLevel>>(|| None);
    let name_query = use_signal(String::new);
    let time_from = use_signal(String::new);
    let time_to = use_signal(String::new);
    let mut filter_open = use_signal(|| false);
    let mut page = use_signal(|| 1usize);
    let view = use_signal(|| ViewMode::Bars);

    let records_resource = {
        let pid = player_id.clone();
        use_resource(move || {
            let pid = pid.clone();
            let tf = time_from();
            let tt = time_to();
            async move {
                let f = GachaFilter {
                    card_pool: Some(active_pool),
                    quality_level: quality(),
                    name: {
                        let n = name_query();
                        if n.is_empty() { None } else { Some(n) }
                    },
                    time_from: parse_day(&tf, false),
                    time_to: parse_day(&tt, true),
                    limit: None,
                    offset: None,
                };
                api::query(&pid, &f).await
            }
        })
    };

    let records: Vec<GachaRecord> = match &*records_resource.read() {
        Some(Ok(v)) => v.clone(),
        _ => Vec::new(),
    };
    let loading = records_resource.read().is_none();
    let chrono = enrich_pulls(records);
    let stats = banner_stats(&chrono);
    let total_pages = chrono.len().div_ceil(PAGE_SIZE).max(1);
    let page_slice: Vec<EnrichedPull> = chrono
        .iter()
        .skip((page() - 1) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .cloned()
        .collect();

    // Pre-format values that can't be interpolated lazily in rsx.
    let per_up_display = if stats.up_count == 0 {
        "—".to_string()
    } else {
        format!("{:.1}", stats.total as f64 / stats.up_count as f64)
    };
    let stray_display = format!("{}/{}", stats.stray_count, stats.r5.len());
    let pool_title = card_pool_label(active_pool);

    let view_now = view();

    rsx! {
        div { class: "flex flex-col gap-4 h-full",
            // Header: pool title + filter toggle
            div { class: "flex items-center justify-between",
                h2 { class: "text-lg text-star-5", "{pool_title}" }
                button {
                    class: "text-sm px-2 py-1 rounded border border-border-gold/40 text-fg-base hover:bg-border-gold/10",
                    onclick: move |_| filter_open.set(!filter_open()),
                    if filter_open() { "▲ 筛选" } else { "▼ 筛选" }
                }
            }

            FilterPanel {
                quality,
                name_query,
                time_from,
                time_to,
                open: filter_open(),
            }

            // View tabs
            div { class: "flex gap-2",
                ViewTab { mode: ViewMode::Bars,    label: "条形式",   view }
                ViewTab { mode: ViewMode::Cards,   label: "卡片式",   view }
                ViewTab { mode: ViewMode::Summary, label: "版本总结", view }
            }

            // Stats strip
            div { class: "grid grid-cols-4 gap-3",
                StatCell { value: format!("{}", stats.total),    label: "抽卡数".to_string() }
                StatCell { value: format!("{}", stats.up_count), label: "UP 数".to_string() }
                StatCell { value: per_up_display,                label: "每 UP 抽数".to_string() }
                StatCell { value: stray_display,                 label: "歪/出卡数".to_string() }
            }

            // Graphical view slot
            div { class: "min-h-40",
                match view_now {
                    ViewMode::Bars    => rsx! { components::bars_view::BarsView    { pulls: chrono.clone() } },
                    ViewMode::Cards   => rsx! { components::cards_view::CardsView  { pulls: chrono.clone() } },
                    ViewMode::Summary => rsx! { components::summary_view::SummaryView { pulls: chrono.clone() } },
                }
            }

            // Table + pagination
            RecordTable { pulls: page_slice, loading }
            Pagination {
                page: page(),
                total_pages,
                on_page: move |p| page.set(p),
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ViewTabProps {
    mode: ViewMode,
    label: &'static str,
    view: Signal<ViewMode>,
}

#[component]
fn ViewTab(mut props: ViewTabProps) -> Element {
    let active = *props.view.read() == props.mode;
    let cls = if active {
        "px-3 py-1 rounded bg-border-gold text-bg-base text-sm"
    } else {
        "px-3 py-1 rounded border border-border-gold/40 text-fg-base hover:bg-border-gold/10 text-sm"
    };
    rsx! {
        button { class: "{cls}",
            onclick: move |_| props.view.set(props.mode),
            "{props.label}"
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct StatCellProps {
    value: String,
    label: String,
}

#[component]
fn StatCell(props: StatCellProps) -> Element {
    rsx! {
        div { class: "bg-bg-panel border border-border-gold/30 rounded p-3 flex flex-col items-center",
            div { class: "text-xl text-star-5", "{props.value}" }
            div { class: "text-xs text-fg-dim mt-1", "{props.label}" }
        }
    }
}

fn parse_day(s: &str, end_of_day: bool) -> Option<chrono::NaiveDateTime> {
    if s.is_empty() {
        return None;
    }
    let date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()?;
    let t = if end_of_day {
        date.and_hms_opt(23, 59, 59)?
    } else {
        date.and_hms_opt(0, 0, 0)?
    };
    Some(t)
}
