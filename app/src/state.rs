use std::path::PathBuf;

use wuwa_gacha_history::SnifferHandle;

/// App-wide context; cheap to clone (everything inside is `Arc` or `PathBuf`).
#[derive(Clone)]
pub struct AppCtx {
    pub sniffer_ca_dir: PathBuf,
    pub sniffer: SnifferHandle,
}

impl AppCtx {
    pub fn init() -> Self {
        Self {
            sniffer_ca_dir: crate::platform::sniffer_ca_dir(),
            sniffer: SnifferHandle::new(),
        }
    }
}
