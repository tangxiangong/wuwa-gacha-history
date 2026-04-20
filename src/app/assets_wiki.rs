//! Generated-match-table accessor for wiki-art portraits.
//!
//! The match body is produced by `build.rs` at compile time from
//! `assets/wiki-art/{characters,weapons}/*.png`. New PNGs dropped in those
//! directories are picked up automatically on the next build.

include!(concat!(env!("OUT_DIR"), "/wiki_art_match.rs"));
