//! Ensure `assets/tailwind.css` exists as a placeholder so `cargo check` can
//! resolve the `asset!("/assets/tailwind.css")` macro even on a fresh clone.
//! `dx serve` / `dx build` overwrites this placeholder with the real compiled
//! Tailwind output. The file is gitignored.

use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"),
    );
    let out_css = manifest_dir.join("assets").join("tailwind.css");
    if !out_css.exists() {
        fs::write(
            &out_css,
            "/* placeholder — `dx serve` regenerates this from tailwind.css */\n",
        )
        .expect("write tailwind.css stub");
    }
    println!("cargo:rerun-if-changed=tailwind.css");
}
