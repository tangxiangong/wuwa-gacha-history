use std::path::PathBuf;
use std::time::Duration;

use dioxus::prelude::*;
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::{api, platform},
    app::components::GlobalState,
    app::state::AppCtx,
    core::{CardPool, CapturedParams, LogParams, SnifferEvent},
};

const ALL_POOLS: [CardPool; 7] = [
    CardPool::FeaturedResonatorConvene,
    CardPool::FeaturedWeaponConvene,
    CardPool::StandardResonatorConvene,
    CardPool::StandardWeaponConvene,
    CardPool::NoviceConvene,
    CardPool::BeginnerChoiceConvene,
    CardPool::GivebackCustomConvene,
];

const SNIFFER_TIMEOUT_MS: u64 = 180_000;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PastedPayload {
    player_id: String,
    server_id: String,
    language_code: String,
    record_id: String,
}

#[derive(Clone, PartialEq)]
enum Activity {
    Idle,
    Fetching,
    Sniffing,
    ReadingLog,
}

#[derive(Props, Clone, PartialEq)]
pub struct FetchFormProps {
    pub on_success: EventHandler<String>,
    #[props(default)]
    pub initial_json: Option<String>,
}

#[component]
pub fn FetchForm(props: FetchFormProps) -> Element {
    let ctx = use_context::<AppCtx>();
    let state = use_context::<GlobalState>();

    let mut json = use_signal(|| props.initial_json.clone().unwrap_or_default());
    let mut activity = use_signal(|| Activity::Idle);
    let mut status = use_signal(String::new);
    let mut error = use_signal(String::new);

    // Sync incoming initial_json if parent changes it.
    use_effect({
        let incoming = props.initial_json.clone();
        move || {
            if let Some(s) = incoming.clone()
                && !s.is_empty()
                && json() != s
            {
                json.set(s);
            }
        }
    });

    // Subscribe to sniffer events while mounted.
    {
        let sniffer = ctx.sniffer.clone();
        let mut json = json;
        let mut status = status;
        let mut activity = activity;
        use_future(move || {
            let mut rx = sniffer.subscribe();
            let sniffer = sniffer.clone();
            async move {
                while let Ok(evt) = rx.recv().await {
                    match evt {
                        SnifferEvent::Captured(p) => {
                            json.set(captured_to_json(&p));
                            status.set(format!("已捕获玩家 {} 的参数（抓包）", p.player_id));
                            activity.set(Activity::Idle);
                            // Stop after first capture.
                            let s = sniffer.clone();
                            spawn(async move {
                                let _ = s.stop().await;
                            });
                        }
                        SnifferEvent::Started { port } => {
                            status.set(format!(
                                "代理已启动（127.0.0.1:{port}），请打开游戏 → 抽卡 → 历史记录"
                            ));
                        }
                        SnifferEvent::Stopped => {
                            if matches!(activity(), Activity::Sniffing) {
                                activity.set(Activity::Idle);
                            }
                        }
                        SnifferEvent::Error(e) => {
                            error.set(format!("抓包错误：{e}"));
                            activity.set(Activity::Idle);
                        }
                    }
                }
            }
        });
    }

    let busy = matches!(
        activity(),
        Activity::Fetching | Activity::Sniffing | Activity::ReadingLog
    );

    // ----- handlers -----

    // Read game log. If `force_pick` is true, always prompt dir picker; else use cached dir if any.
    let read_log = {
        let saved_game_dir = state.saved_game_dir;
        move |force_pick: bool| {
            let mut saved_game_dir = saved_game_dir;
            spawn(async move {
                error.set(String::new());
                status.set(String::new());

                let dir: Option<PathBuf> = if force_pick || saved_game_dir.read().is_none() {
                    platform::pick_directory().await
                } else {
                    saved_game_dir.read().clone()
                };

                let Some(dir) = dir else {
                    return;
                };
                saved_game_dir.set(Some(dir.clone()));
                activity.set(Activity::ReadingLog);
                match api::read_params(None, Some(dir)).await {
                    Ok(p) => {
                        json.set(log_params_to_json(&p));
                        status.set(format!("已从日志读取玩家 {} 的参数", p.player_id));
                    }
                    Err(e) => error.set(e),
                }
                activity.set(Activity::Idle);
            });
        }
    };

    let handle_read_log = move |_: Event<MouseData>| read_log(false);

    let handle_pick_game_dir = move |_: Event<MouseData>| read_log(true);

    let toggle_sniff = {
        let sniffer = ctx.sniffer.clone();
        let ca_dir = ctx.sniffer_ca_dir.clone();
        move |_: Event<MouseData>| {
            let sniffer = sniffer.clone();
            let ca_dir = ca_dir.clone();
            let running = matches!(activity(), Activity::Sniffing);
            spawn(async move {
                error.set(String::new());
                if running {
                    status.set("正在停止代理…".into());
                    let _ = sniffer.stop().await;
                    activity.set(Activity::Idle);
                    status.set("已取消监听".into());
                } else {
                    activity.set(Activity::Sniffing);
                    status.set(String::new());
                    match sniffer.start(ca_dir).await {
                        Ok(_) => {
                            // Start timeout watchdog.
                            let sniffer_for_timeout = sniffer.clone();
                            spawn(async move {
                                tokio::time::sleep(Duration::from_millis(SNIFFER_TIMEOUT_MS))
                                    .await;
                                // If still sniffing, stop with timeout error.
                                if matches!(activity(), Activity::Sniffing) {
                                    let _ = sniffer_for_timeout.stop().await;
                                    error.set("超时未捕获到请求，已停止代理".into());
                                    activity.set(Activity::Idle);
                                }
                            });
                        }
                        Err(e) => {
                            error.set(e);
                            activity.set(Activity::Idle);
                        }
                    }
                }
            });
        }
    };

    let handle_submit = move |_: Event<MouseData>| {
        let on_success = props.on_success;
        let raw_now = json();
        spawn(async move {
            error.set(String::new());
            status.set(String::new());
            let payload: PastedPayload = match serde_json::from_str(raw_now.trim()) {
                Ok(p) => p,
                Err(e) => {
                    error.set(format!("JSON 解析失败：{e}"));
                    return;
                }
            };
            if !is_9_digits(&payload.player_id) {
                error.set("playerId 必须是 9 位数字".into());
                return;
            }
            activity.set(Activity::Fetching);
            match api::fetch_all_pools(
                payload.player_id.clone(),
                payload.server_id,
                payload.language_code,
                payload.record_id,
                ALL_POOLS.to_vec(),
            )
            .await
            {
                Ok(n) => {
                    status.set(format!("已写入 {n} 条记录"));
                    on_success.call(payload.player_id);
                }
                Err(e) => error.set(e.to_string()),
            }
            activity.set(Activity::Idle);
        });
    };

    let sniff_label = if matches!(activity(), Activity::Sniffing) {
        "取消监听"
    } else {
        "抓包获取"
    };
    let read_label = if matches!(activity(), Activity::ReadingLog) {
        "读取中…"
    } else {
        "从日志获取"
    };
    let submit_label = if matches!(activity(), Activity::Fetching) {
        "获取中..."
    } else {
        "获取记录"
    };
    let game_dir_hint = state
        .saved_game_dir
        .read()
        .clone()
        .map(|p| p.to_string_lossy().to_string());
    let json_empty = json().trim().is_empty();
    let submit_disabled = busy || json_empty;

    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            // Helpers row
            div { class: "flex flex-wrap items-center gap-2",
                span { class: "text-fg-dim text-xs mr-1", "自动填充参数：" }
                button {
                    class: "px-3 py-1 rounded border border-border-gold/60 text-star-5 hover:bg-border-gold/10 text-sm disabled:opacity-40 disabled:cursor-not-allowed",
                    disabled: busy,
                    onclick: handle_read_log,
                    title: "从游戏日志提取抽卡参数",
                    "{read_label}"
                }
                button {
                    class: "px-3 py-1 rounded border border-border-gold/60 text-star-5 hover:bg-border-gold/10 text-sm disabled:opacity-40 disabled:cursor-not-allowed",
                    disabled: busy,
                    onclick: handle_pick_game_dir,
                    title: "选择鸣潮游戏安装目录",
                    "选择游戏目录…"
                }
                button {
                    class: "px-3 py-1 rounded border border-border-gold/60 text-star-5 hover:bg-border-gold/10 text-sm disabled:opacity-40 disabled:cursor-not-allowed",
                    disabled: matches!(activity(), Activity::Fetching | Activity::ReadingLog),
                    onclick: toggle_sniff,
                    title: "启动本地 MITM 代理抓取游戏请求（需授权证书）",
                    "{sniff_label}"
                }
            }

            // Textarea
            textarea {
                class: "bg-bg-panel border border-border-gold/60 rounded p-2 h-36 text-fg-base text-sm font-mono disabled:opacity-50",
                placeholder: "粘贴 JSON，例如 {{\"playerId\":\"123456789\",\"serverId\":\"...\",\"languageCode\":\"zh-Hans\",\"recordId\":\"...\"}}",
                disabled: busy,
                value: "{json}",
                oninput: move |e| json.set(e.value()),
            }

            if let Some(dir) = game_dir_hint {
                p { class: "text-fg-dim text-xs break-all", "已记住游戏目录：{dir}" }
            }
            if !status().is_empty() {
                p { class: "text-star-4 text-xs", "{status}" }
            }
            if !error().is_empty() {
                p { class: "text-red-400 text-xs", "{error}" }
            }

            // Submit
            button {
                class: "w-full bg-border-gold text-bg-base px-4 py-2 rounded hover:opacity-90 disabled:opacity-50 transition-colors",
                disabled: submit_disabled,
                onclick: handle_submit,
                "{submit_label}"
            }
        }
    }
}

fn is_9_digits(s: &str) -> bool {
    s.len() == 9 && s.bytes().all(|b| b.is_ascii_digit())
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
