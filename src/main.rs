pub mod app;
pub mod core;

use dioxus::prelude::*;

fn main() {
    LaunchBuilder::desktop().launch(app::App);
}
