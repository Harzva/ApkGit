mod api;
mod app;
mod data;
mod download;

use eframe::{egui, NativeOptions};

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ReleaseMarket v0.1.1",
        options,
        Box::new(|_cc| Ok(Box::new(app::ReleaseMarketApp::default()))),
    )
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    use eframe::NativeOptions;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    let options = NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "ReleaseMarket",
        options,
        Box::new(|_cc| Ok(Box::new(app::ReleaseMarketApp::default()))),
    )
    .unwrap();
}

#[cfg(target_os = "android")]
pub use android_logger;
