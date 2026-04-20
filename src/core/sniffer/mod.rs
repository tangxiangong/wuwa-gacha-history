//! Local MITM HTTP proxy that captures gacha-record request params.
//! Exposed as a reusable handle with a tokio broadcast channel so that
//! any UI frontend (Dioxus, CLI, etc.) can subscribe.

mod ca;
mod interceptor;
mod proxy;

use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;
use std::sync::Arc;

use hudsucker::{Proxy, certificate_authority::RcgenAuthority};
use tokio::sync::{Mutex, broadcast, mpsc, oneshot};

pub use interceptor::CapturedParams;

const EVENT_CHANNEL_CAPACITY: usize = 64;

#[derive(Debug, Clone)]
pub enum SnifferEvent {
    Started { port: u16 },
    Stopped,
    Captured(CapturedParams),
    Error(String),
}

/// Cheap-clone handle. Internal state is `Arc`-wrapped; subscribers get their
/// own `broadcast::Receiver` via [`Self::subscribe`].
#[derive(Clone)]
pub struct SnifferHandle {
    inner: Arc<Mutex<Option<RunningSniffer>>>,
    events: broadcast::Sender<SnifferEvent>,
}

struct RunningSniffer {
    shutdown: Option<oneshot::Sender<()>>,
    proxy_guard: Option<proxy::ProxyGuard>,
}

impl SnifferHandle {
    pub fn new() -> Self {
        let (events, _rx) = broadcast::channel(EVENT_CHANNEL_CAPACITY);
        Self {
            inner: Arc::new(Mutex::new(None)),
            events,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<SnifferEvent> {
        self.events.subscribe()
    }

    pub async fn start(&self, ca_dir: PathBuf) -> Result<u16, String> {
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

        // Fan mpsc captures out to broadcast subscribers.
        let fan_events = self.events.clone();
        tokio::spawn(async move {
            while let Some(params) = rx.recv().await {
                let _ = fan_events.send(SnifferEvent::Captured(params));
            }
        });

        let proxy_events = self.events.clone();
        tokio::spawn(async move {
            if let Err(e) = proxy_built.start().await {
                tracing::error!("proxy error: {e}");
                let _ = proxy_events.send(SnifferEvent::Error(format!("proxy error: {e}")));
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

        let _ = self.events.send(SnifferEvent::Started { port });
        Ok(port)
    }

    pub async fn stop(&self) -> Result<(), String> {
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
        let _ = self.events.send(SnifferEvent::Stopped);
        Ok(())
    }
}

impl Default for SnifferHandle {
    fn default() -> Self {
        Self::new()
    }
}

fn pick_free_port() -> Result<u16, String> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    let port = listener.local_addr().map_err(|e| e.to_string())?.port();
    drop(listener);
    Ok(port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::broadcast::error::TryRecvError;

    #[tokio::test]
    async fn new_handle_exposes_usable_channel() {
        let h = SnifferHandle::new();
        let mut rx = h.subscribe();
        // No events yet.
        assert!(matches!(rx.try_recv(), Err(TryRecvError::Empty)));

        // Send a synthetic event and verify reception.
        h.events.send(SnifferEvent::Stopped).unwrap();
        let evt = rx.recv().await.unwrap();
        assert!(matches!(evt, SnifferEvent::Stopped));
    }

    #[tokio::test]
    async fn double_stop_is_ok() {
        let h = SnifferHandle::new();
        assert!(h.stop().await.is_ok());
        assert!(h.stop().await.is_ok());
    }
}
