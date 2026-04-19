use dioxus::prelude::*;
use serde::Deserialize;
use wuwa_gacha_history::CardPool;

use crate::api;

const ALL_POOLS: [CardPool; 7] = [
    CardPool::FeaturedResonatorConvene,
    CardPool::FeaturedWeaponConvene,
    CardPool::StandardResonatorConvene,
    CardPool::StandardWeaponConvene,
    CardPool::NoviceConvene,
    CardPool::BeginnerChoiceConvene,
    CardPool::GivebackCustomConvene,
];

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PastedPayload {
    player_id: String,
    server_id: String,
    language_code: String,
    record_id: String,
}

#[derive(Clone, PartialEq)]
enum Status {
    Idle,
    Validating,
    Fetching,
    Success(u64),
    Error(String),
}

#[derive(Props, Clone, PartialEq)]
pub struct FetchFormProps {
    pub on_success: EventHandler<String>,
    #[props(default)]
    pub initial_json: Option<String>,
}

#[component]
pub fn FetchForm(props: FetchFormProps) -> Element {
    let mut raw = use_signal(|| props.initial_json.clone().unwrap_or_default());
    let mut status = use_signal(|| Status::Idle);

    // Sync incoming `initial_json` if parent changes it (e.g., sniffer captured params).
    use_effect({
        let incoming = props.initial_json.clone();
        move || {
            if let Some(s) = incoming.clone() {
                if !s.is_empty() && raw() != s {
                    raw.set(s);
                }
            }
        }
    });

    let on_submit = {
        let on_success = props.on_success.clone();
        move |_: Event<MouseData>| {
            let raw_now = raw();
            let on_success = on_success.clone();
            spawn(async move {
                status.set(Status::Validating);
                let payload: PastedPayload = match serde_json::from_str(raw_now.trim()) {
                    Ok(p) => p,
                    Err(e) => {
                        status.set(Status::Error(format!("JSON 解析失败：{e}")));
                        return;
                    }
                };
                if !is_9_digits(&payload.player_id) {
                    status.set(Status::Error("playerId 必须是 9 位数字".into()));
                    return;
                }
                status.set(Status::Fetching);
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
                        status.set(Status::Success(n));
                        on_success.call(payload.player_id);
                    }
                    Err(e) => status.set(Status::Error(e.to_string())),
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col gap-3 w-full max-w-xl",
            label { class: "text-fg-dim text-sm", "粘贴抽卡参数 JSON：" }
            textarea {
                class: "bg-bg-panel border border-border-gold rounded p-2 h-40 text-fg-base text-sm font-mono",
                value: "{raw}",
                oninput: move |e| raw.set(e.value()),
            }
            button {
                class: "bg-border-gold text-bg-base px-4 py-2 rounded hover:opacity-90 disabled:opacity-50 transition-colors",
                disabled: matches!(status(), Status::Fetching | Status::Validating),
                onclick: on_submit,
                {
                    match status() {
                        Status::Idle | Status::Validating => "拉取记录".to_string(),
                        Status::Fetching => "正在拉取…".to_string(),
                        Status::Success(n) => format!("已写入 {n} 条，点此重拉"),
                        Status::Error(_) => "重试".to_string(),
                    }
                }
            }
            if let Status::Error(msg) = status() {
                div { class: "text-red-400 text-sm", "{msg}" }
            }
        }
    }
}

fn is_9_digits(s: &str) -> bool {
    s.len() == 9 && s.bytes().all(|b| b.is_ascii_digit())
}
