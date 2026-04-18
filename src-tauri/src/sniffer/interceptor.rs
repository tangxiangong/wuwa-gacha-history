use http_body_util::{BodyExt, Full};
use hudsucker::{
    Body, HttpContext, HttpHandler, RequestOrResponse,
    hyper::{Request, Response},
};
use serde::Deserialize;
use tokio::sync::mpsc;

pub const TARGET_PATH: &str = "/gacha/record/query";

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedParams {
    pub player_id: String,
    pub server_id: String,
    pub language_code: String,
    pub record_id: String,
    pub card_pool_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawBody {
    player_id: String,
    server_id: String,
    language_code: String,
    record_id: String,
    #[serde(default)]
    card_pool_id: String,
}

#[derive(Clone)]
pub struct Interceptor {
    tx: mpsc::UnboundedSender<CapturedParams>,
}

impl Interceptor {
    pub fn new(tx: mpsc::UnboundedSender<CapturedParams>) -> Self {
        Self { tx }
    }
}

fn host_matches(host: &str) -> bool {
    let h = host.to_ascii_lowercase();
    h.ends_with(".aki-game2.com") || h.ends_with(".aki-game2.net")
}

impl HttpHandler for Interceptor {
    async fn should_intercept(&mut self, _ctx: &HttpContext, req: &Request<Body>) -> bool {
        req.uri().host().map(host_matches).unwrap_or(false)
    }

    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        let is_target = req.uri().host().map(host_matches).unwrap_or(false)
            && req.uri().path() == TARGET_PATH;

        if !is_target {
            return req.into();
        }

        let (parts, body) = req.into_parts();
        let bytes = match body.collect().await {
            Ok(b) => b.to_bytes(),
            Err(e) => {
                tracing::warn!("failed to collect body: {e}");
                return Request::from_parts(parts, Body::empty()).into();
            }
        };

        if let Ok(raw) = serde_json::from_slice::<RawBody>(&bytes) {
            let captured = CapturedParams {
                player_id: raw.player_id,
                server_id: raw.server_id,
                language_code: raw.language_code,
                record_id: raw.record_id,
                card_pool_id: raw.card_pool_id,
            };
            let _ = self.tx.send(captured);
        }

        Request::from_parts(parts, Body::from(Full::new(bytes))).into()
    }

    async fn handle_response(
        &mut self,
        _ctx: &HttpContext,
        res: Response<Body>,
    ) -> Response<Body> {
        res
    }
}

#[cfg(test)]
mod tests {
    use super::host_matches;

    #[test]
    fn matches_real_hosts() {
        assert!(host_matches("gmserver-api.aki-game2.com"));
        assert!(host_matches("gmserver-api.aki-game2.net"));
        assert!(host_matches("GMSERVER-API.AKI-GAME2.COM"));
    }

    #[test]
    fn rejects_unrelated() {
        assert!(!host_matches("aki-game2.com"));
        assert!(!host_matches("evil.example.com"));
        assert!(!host_matches("aki-game.com"));
    }
}
