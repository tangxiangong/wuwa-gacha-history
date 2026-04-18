use std::collections::HashMap;
use std::path::PathBuf;

use regex::Regex;
use serde::Serialize;
use tokio::fs;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogParams {
    pub player_id: String,
    pub server_id: String,
    pub language_code: String,
    pub record_id: String,
    pub card_pool_id: Option<String>,
    pub source_path: String,
    pub source_url: String,
}

pub async fn read_params(
    explicit_path: Option<PathBuf>,
    game_dir: Option<PathBuf>,
) -> Result<LogParams, String> {
    let candidates: Vec<PathBuf> = if let Some(p) = explicit_path {
        vec![p]
    } else if let Some(dir) = game_dir {
        paths_under_game_dir(&dir)
    } else {
        return Err(
            "请先点击「选择游戏目录」指定鸣潮安装位置".to_string(),
        );
    };

    let mut best: Option<(PathBuf, String, u64)> = None;
    for path in candidates.into_iter().filter(|p| p.exists()) {
        let content = match fs::read_to_string(&path).await {
            Ok(s) => s,
            Err(_) => continue,
        };
        if let Some(url) = extract_latest_url(&content) {
            let mtime = fs::metadata(&path)
                .await
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let is_newer = best.as_ref().map(|(_, _, t)| mtime >= *t).unwrap_or(true);
            if is_newer {
                best = Some((path, url, mtime));
            }
        }
    }

    let (path, url, _) = best.ok_or_else(|| {
        "未在日志中找到抽卡链接，请先在游戏内打开抽卡记录并翻页，或手动选择日志文件".to_string()
    })?;

    let mut params = parse_url(&url)?;
    params.source_path = path.to_string_lossy().into_owned();
    params.source_url = url;
    Ok(params)
}

fn extract_latest_url(content: &str) -> Option<String> {
    let re = Regex::new(
        r#"https?://aki-gm-resources(?:-oversea)?\.aki-game\.(?:com|net)/[^\s"'\\]+"#,
    )
    .ok()?;
    re.find_iter(content).last().map(|m| m.as_str().to_string())
}

fn parse_url(url: &str) -> Result<LogParams, String> {
    let query = url
        .rsplit_once('?')
        .map(|(_, q)| q)
        .ok_or_else(|| "链接中无查询参数".to_string())?;

    let mut pairs: HashMap<&str, String> = HashMap::new();
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let decoded = percent_decode(v);
            pairs.insert(k, decoded);
        }
    }

    let take = |key: &str| -> Result<String, String> {
        pairs
            .get(key)
            .cloned()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| format!("链接中缺少 {key}"))
    };

    Ok(LogParams {
        player_id: take("player_id")?,
        server_id: take("svr_id")?,
        language_code: take("lang")?,
        record_id: take("record_id").or_else(|_| take("resources_id"))?,
        card_pool_id: pairs.get("gacha_id").cloned().filter(|s| !s.is_empty()),
        source_path: String::new(),
        source_url: String::new(),
    })
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hi = hex_val(bytes[i + 1]);
                let lo = hex_val(bytes[i + 2]);
                match (hi, lo) {
                    (Some(h), Some(l)) => {
                        out.push((h << 4) | l);
                        i += 3;
                    }
                    _ => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn paths_under_game_dir(game_dir: &std::path::Path) -> Vec<PathBuf> {
    let rel_main: &[&str] = &["Client", "Saved", "Logs", "Client.log"];
    let rel_debug: &[&str] = &[
        "Client",
        "Binaries",
        "Win64",
        "ThirdParty",
        "KrPcSdk_Global",
        "KRSDKRes",
        "KRSDKWebView",
        "debug.log",
    ];
    vec![
        rel_main.iter().fold(game_dir.to_path_buf(), |p, s| p.join(s)),
        rel_debug.iter().fold(game_dir.to_path_buf(), |p, s| p.join(s)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_overseas_url() {
        let url = "https://aki-gm-resources-oversea.aki-game.net/aki/gacha/index.html#/record?svr_id=76402e5b20be2837ef4fd4e4642b5b54&player_id=123456789&lang=en&svr_area=global&record_id=abcdef&resources_id=xxx&gacha_id=pool-uuid&gacha_type=1";
        let p = parse_url(url).unwrap();
        assert_eq!(p.player_id, "123456789");
        assert_eq!(p.server_id, "76402e5b20be2837ef4fd4e4642b5b54");
        assert_eq!(p.language_code, "en");
        assert_eq!(p.record_id, "abcdef");
        assert_eq!(p.card_pool_id.as_deref(), Some("pool-uuid"));
    }

    #[test]
    fn extracts_latest_from_log_noise() {
        let log = "garbage garbage\n[2026.01.01] LogHttp: old https://aki-gm-resources.aki-game.com/aki/gacha/index.html#/record?svr_id=s1&player_id=1&lang=zh-Hans&record_id=old\nmore text\n[2026.01.02] \"#url\": \"https://aki-gm-resources-oversea.aki-game.net/aki/gacha/index.html#/record?svr_id=s2&player_id=2&lang=en&record_id=new\"\n";
        let url = extract_latest_url(log).unwrap();
        assert!(url.contains("record_id=new"));
    }
}
