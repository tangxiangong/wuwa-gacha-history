use crate::{app::platform, core::SnifferHandle};
use std::path::PathBuf;

/// App-wide context; cheap to clone (everything inside is `Arc` or `PathBuf`).
#[derive(Clone)]
pub struct AppCtx {
    pub sniffer_ca_dir: PathBuf,
    pub sniffer: SnifferHandle,
}

impl AppCtx {
    pub fn init() -> Self {
        Self {
            sniffer_ca_dir: platform::sniffer_ca_dir(),
            sniffer: SnifferHandle::new(),
        }
    }
}
