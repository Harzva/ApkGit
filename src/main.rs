use eframe::{egui, NativeOptions};
use gitmarket::app::GitMarketApp;

fn main() -> eframe::Result<()> {
    let icon =
        eframe::icon_data::from_png_bytes(include_bytes!("../assets/brand/gitmarket-logo-512.png"))
            .ok();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([760.0, 520.0])
            .with_icon(icon.unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "GitMarket v0.1.7",
        options,
        Box::new(|_cc| Ok(Box::new(GitMarketApp::default()))),
    )
}
