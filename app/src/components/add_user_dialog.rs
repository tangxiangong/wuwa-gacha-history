use dioxus::prelude::*;
use serde_json::json;
use wuwa_gacha_history::{CapturedParams, LogParams, SnifferEvent};

use crate::components::fetch_form::FetchForm;
use crate::platform;
use crate::state::AppCtx;

#[derive(Clone, PartialEq)]
enum SnifferStatus {
    Idle,
    Running { port: u16 },
    Captured,
    Error(String),
}

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

    let ctx = use_context::<AppCtx>();
    let mut prefilled_json = use_signal(String::new);
    let mut sniffer_status = use_signal(|| SnifferStatus::Idle);

    // Subscribe to sniffer events. The future runs while this component is mounted.
    {
        let sniffer = ctx.sniffer.clone();
        use_future(move || {
            let mut rx = sniffer.subscribe();
            async move {
                while let Ok(evt) = rx.recv().await {
                    match evt {
                        SnifferEvent::Captured(p) => {
                            prefilled_json.set(captured_to_json(&p));
                            sniffer_status.set(SnifferStatus::Captured);
                        }
                        SnifferEvent::Started { port } => {
                            sniffer_status.set(SnifferStatus::Running { port });
                        }
                        SnifferEvent::Stopped => sniffer_status.set(SnifferStatus::Idle),
                        SnifferEvent::Error(e) => sniffer_status.set(SnifferStatus::Error(e)),
                    }
                }
            }
        });
    }

    // Toggle sniffer start/stop
    let toggle_sniffer = {
        let sniffer = ctx.sniffer.clone();
        let ca_dir = ctx.sniffer_ca_dir.clone();
        move |_: Event<MouseData>| {
            let sniffer = sniffer.clone();
            let ca_dir = ca_dir.clone();
            let running = matches!(
                sniffer_status(),
                SnifferStatus::Running { .. } | SnifferStatus::Captured
            );
            spawn(async move {
                if running {
                    let _ = sniffer.stop().await;
                } else {
                    match sniffer.start(ca_dir).await {
                        Ok(port) => sniffer_status.set(SnifferStatus::Running { port }),
                        Err(e) => sniffer_status.set(SnifferStatus::Error(e)),
                    }
                }
            });
        }
    };

    let use_log_reader = move |_: Event<MouseData>| {
        spawn(async move {
            let dir = platform::pick_directory().await;
            if dir.is_none() {
                return;
            }
            match crate::api::read_params(None, dir).await {
                Ok(p) => prefilled_json.set(log_params_to_json(&p)),
                Err(e) => sniffer_status.set(SnifferStatus::Error(e)),
            }
        });
    };

    // Derived display values
    let sniffer_button_label = match sniffer_status() {
        SnifferStatus::Running { .. } | SnifferStatus::Captured => "停止抓包",
        SnifferStatus::Idle => "开启抓包",
        SnifferStatus::Error(_) => "重试抓包",
    };

    rsx! {
        div {
            class: "fixed inset-0 bg-black/60 flex items-center justify-center z-50",
            onclick: move |_| props.on_close.call(()),
            div {
                class: "bg-bg-panel border border-border-gold rounded p-6 w-[36rem] max-w-[90vw] flex flex-col gap-4",
                onclick: move |e| e.stop_propagation(),

                h2 { class: "text-lg text-star-5", "添加用户" }

                // Auto-fill helpers
                div { class: "flex gap-2",
                    button {
                        class: "px-3 py-1 rounded border border-border-gold text-star-5 hover:bg-border-gold/20 text-sm",
                        onclick: toggle_sniffer,
                        "{sniffer_button_label}"
                    }
                    button {
                        class: "px-3 py-1 rounded border border-border-gold text-star-5 hover:bg-border-gold/20 text-sm",
                        onclick: use_log_reader,
                        "从游戏日志读取"
                    }
                }

                if let SnifferStatus::Error(msg) = sniffer_status() {
                    div { class: "text-red-400 text-sm", "{msg}" }
                }
                if let SnifferStatus::Running { port } = sniffer_status() {
                    div {
                        class: "text-fg-dim text-sm",
                        "抓包代理在 127.0.0.1:{port} 运行中；请在游戏内打开抽卡记录"
                    }
                }

                FetchForm {
                    initial_json: Some(prefilled_json()),
                    on_success: move |pid: String| {
                        props.on_user_added.call(pid);
                        props.on_close.call(());
                    },
                }

                button {
                    class: "self-end text-fg-dim hover:text-fg-base text-sm",
                    onclick: move |_| props.on_close.call(()),
                    "关闭"
                }
            }
        }
    }
}

fn captured_to_json(p: &CapturedParams) -> String {
    serde_json::to_string_pretty(&json!({
        "playerId": p.player_id,
        "serverId": p.server_id,
        "languageCode": p.language_code,
        "recordId": p.record_id,
    }))
    .unwrap_or_default()
}

fn log_params_to_json(p: &LogParams) -> String {
    serde_json::to_string_pretty(&json!({
        "playerId": p.player_id,
        "serverId": p.server_id,
        "languageCode": p.language_code,
        "recordId": p.record_id,
    }))
    .unwrap_or_default()
}
