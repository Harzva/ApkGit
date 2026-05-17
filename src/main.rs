#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

use eframe::{egui, NativeOptions};
use gitmarket::app::GitMarketApp;

fn main() -> eframe::Result<()> {
    let icon =
        eframe::icon_data::from_png_bytes(include_bytes!("../assets/brand/gitmarket-logo-512.png"))
            .ok();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([980.0, 640.0])
            .with_icon(icon.unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "GitMarket v0.2.3",
        options,
        Box::new(|_cc| Ok(Box::new(GitMarketApp::default()))),
    )
}
