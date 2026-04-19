use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use directories::ProjectDirs;

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Resolve the per-user application data directory, creating it if absent.
/// Falls back to a subdirectory of `std::env::temp_dir()` if the OS has no
/// project-dirs convention (very unusual).
pub fn data_dir() -> &'static Path {
    DATA_DIR.get_or_init(|| {
        let d = ProjectDirs::from("dev", "tangxiangong", "wuwa-gacha-history")
            .map(|pd| pd.data_dir().to_path_buf())
            .unwrap_or_else(|| std::env::temp_dir().join("wuwa-gacha-history"));
        std::fs::create_dir_all(&d).expect("failed to create app data dir");
        d
    })
}

pub fn db_path() -> PathBuf {
    data_dir().join("gacha.db")
}

pub fn sniffer_ca_dir() -> PathBuf {
    data_dir().join("mitm")
}

/// Spawn a native save dialog and return the chosen path (or None if cancelled).
pub async fn pick_save_file(
    default_name: &str,
    filters: &[(&str, &[&str])],
) -> Option<PathBuf> {
    let mut dlg = rfd::AsyncFileDialog::new().set_file_name(default_name);
    for (desc, exts) in filters {
        dlg = dlg.add_filter(*desc, exts);
    }
    dlg.save_file().await.map(|h| h.path().to_path_buf())
}

/// Spawn a native folder-pick dialog (used for "select game install dir").
pub async fn pick_directory() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .pick_folder()
        .await
        .map(|h| h.path().to_path_buf())
}
