use dioxus::prelude::*;
use wuwa_gacha_history::GachaFilter;

use crate::components::GlobalState;
use crate::platform;

#[derive(Clone, Copy, PartialEq)]
enum ExportFmt {
    Csv,
    Xlsx,
    Json,
}

#[derive(Clone, PartialEq)]
enum ExportStatus {
    Idle,
    Running,
    Ok(String),
    Error(String),
}

#[component]
pub fn ExportDialog() -> Element {
    let mut state = use_context::<GlobalState>();
    if !*state.export_open.read() {
        return rsx! {};
    }

    let fmt = use_signal(|| ExportFmt::Csv);
    let mut status = use_signal(|| ExportStatus::Idle);

    let pid_opt = state.player_id.read().clone();
    let pool_opt = *state.active_pool.read();

    let on_confirm = {
        let pid_opt = pid_opt.clone();
        move |_: Event<MouseData>| {
            let Some(pid) = pid_opt.clone() else {
                status.set(ExportStatus::Error("未选择用户".into()));
                return;
            };
            let chosen = fmt();
            let pool = pool_opt;
            spawn(async move {
                let (default_name, desc, exts): (&str, &str, &[&str]) = match chosen {
                    ExportFmt::Csv => ("gacha.csv", "CSV", &["csv"]),
                    ExportFmt::Xlsx => ("gacha.xlsx", "XLSX", &["xlsx"]),
                    ExportFmt::Json => ("gacha.json", "JSON", &["json"]),
                };
                let Some(path) = platform::pick_save_file(default_name, &[(desc, exts)]).await
                else {
                    return;
                };
                status.set(ExportStatus::Running);
                let filter = GachaFilter {
                    card_pool: pool,
                    ..Default::default()
                };
                match crate::api::export(&pid, &filter, &path).await {
                    Ok(()) => status.set(ExportStatus::Ok(path.to_string_lossy().to_string())),
                    Err(e) => status.set(ExportStatus::Error(e.to_string())),
                }
            });
        }
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black/60 flex items-center justify-center z-50",
            onclick: move |_| state.export_open.set(false),
            div {
                class: "bg-bg-panel border border-border-gold rounded p-6 w-80 flex flex-col gap-4",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-lg text-star-5", "导出" }

                div { class: "flex gap-2",
                    FmtChip { label: "CSV",  this: ExportFmt::Csv,  current: fmt }
                    FmtChip { label: "XLSX", this: ExportFmt::Xlsx, current: fmt }
                    FmtChip { label: "JSON", this: ExportFmt::Json, current: fmt }
                }

                {
                    match status() {
                        ExportStatus::Idle => rsx! {},
                        ExportStatus::Running => rsx! {
                            div { class: "text-fg-dim text-sm", "导出中…" }
                        },
                        ExportStatus::Ok(path) => rsx! {
                            div { class: "text-star-5 text-sm break-all", "已导出至 {path}" }
                        },
                        ExportStatus::Error(e) => rsx! {
                            div { class: "text-red-400 text-sm break-all", "{e}" }
                        },
                    }
                }

                div { class: "flex justify-end gap-2",
                    button {
                        class: "px-3 py-1 rounded border border-border-gold/40 text-fg-base text-sm",
                        onclick: move |_| state.export_open.set(false),
                        "取消"
                    }
                    button {
                        class: "px-3 py-1 rounded bg-border-gold text-bg-base text-sm",
                        onclick: on_confirm,
                        "选择文件并导出"
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct FmtChipProps {
    label: &'static str,
    this: ExportFmt,
    current: Signal<ExportFmt>,
}

#[component]
fn FmtChip(mut props: FmtChipProps) -> Element {
    let active = *props.current.read() == props.this;
    let cls = if active {
        "px-3 py-1 rounded bg-border-gold text-bg-base text-sm"
    } else {
        "px-3 py-1 rounded border border-border-gold/40 text-fg-base hover:bg-border-gold/10 text-sm"
    };
    rsx! {
        button {
            class: "{cls}",
            onclick: move |_| props.current.set(props.this),
            "{props.label}"
        }
    }
}
