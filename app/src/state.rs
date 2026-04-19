use std::path::PathBuf;

use wuwa_gacha_history::SnifferHandle;

/// App-wide context; cheap to clone (everything inside is `Arc` or `PathBuf`).
#[derive(Clone)]
pub struct AppCtx {
    pub db_path: PathBuf,
    pub sniffer_ca_dir: PathBuf,
    pub sniffer: SnifferHandle,
}

impl AppCtx {
    pub fn init() -> Self {
        Self {
            db_path: crate::platform::db_path(),
            sniffer_ca_dir: crate::platform::sniffer_ca_dir(),
            sniffer: SnifferHandle::new(),
        }
    }
}
