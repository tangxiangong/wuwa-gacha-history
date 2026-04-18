mod ca;
mod interceptor;
mod proxy;

use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::sync::Arc;

use hudsucker::{Proxy, certificate_authority::RcgenAuthority};
use tauri::{AppHandle, Emitter};
use tokio::sync::{Mutex, mpsc, oneshot};

pub use interceptor::CapturedParams;

pub const EVENT_PARAMS_CAPTURED: &str = "sniffer://params-captured";
pub const EVENT_STATUS: &str = "sniffer://status";

pub struct SnifferState {
    inner: Arc<Mutex<Option<RunningSniffer>>>,
}

impl Default for SnifferState {
    fn default() -> Self {
        Self { inner: Arc::new(Mutex::new(None)) }
    }
}

struct RunningSniffer {
    shutdown: Option<oneshot::Sender<()>>,
    proxy_guard: Option<proxy::ProxyGuard>,
}

impl SnifferState {
    pub async fn start(&self, app: AppHandle, ca_dir: PathBuf) -> Result<u16, String> {
        let mut slot = self.inner.lock().await;
        if slot.is_some() {
            return Err("sniffer already running".into());
        }

        let material = ca::CaMaterial::load_or_generate(&ca_dir).await?;
        ca::install_to_system_trust(&material.cert_path).await?;
        let (key_pair, ca_cert) = material.into_key_and_cert()?;
        let ca_auth = RcgenAuthority::new(key_pair, ca_cert, 1_000);

        let port = pick_free_port()?;
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let (tx, mut rx) = mpsc::unbounded_channel::<CapturedParams>();
        let handler = interceptor::Interceptor::new(tx);

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        let proxy_built = Proxy::builder()
            .with_addr(addr)
            .with_rustls_client()
            .with_ca(ca_auth)
            .with_http_handler(handler)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            })
            .build();

        let app_for_events = app.clone();
        tokio::spawn(async move {
            while let Some(params) = rx.recv().await {
                let _ = app_for_events.emit(EVENT_PARAMS_CAPTURED, params);
            }
        });

        tokio::spawn(async move {
            if let Err(e) = proxy_built.start().await {
                tracing::error!("proxy error: {e}");
            }
        });

        let proxy_guard = match proxy::ProxyGuard::enable(port).await {
            Ok(g) => g,
            Err(e) => {
                let _ = shutdown_tx.send(());
                return Err(e);
            }
        };

        *slot = Some(RunningSniffer {
            shutdown: Some(shutdown_tx),
            proxy_guard: Some(proxy_guard),
        });

        let _ = app.emit(EVENT_STATUS, "started");
        Ok(port)
    }

    pub async fn stop(&self, app: AppHandle) -> Result<(), String> {
        let mut slot = self.inner.lock().await;
        let Some(mut running) = slot.take() else {
            return Ok(());
        };
        if let Some(guard) = running.proxy_guard.take() {
            let _ = guard.disable().await;
        }
        if let Some(tx) = running.shutdown.take() {
            let _ = tx.send(());
        }
        let _ = app.emit(EVENT_STATUS, "stopped");
        Ok(())
    }
}

fn pick_free_port() -> Result<u16, String> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    drop(listener);
    Ok(port)
}
