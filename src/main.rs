use eframe::{egui, NativeOptions};
use gitmarket::app::GitMarketApp;

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "GitMarket v0.1.2",
        options,
        Box::new(|_cc| Ok(Box::new(GitMarketApp::default()))),
    )
}
