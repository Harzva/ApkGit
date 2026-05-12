pub mod api;
pub mod app;
pub mod data;
pub mod download;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(android_app: android_activity::AndroidApp) {
    use eframe::NativeOptions;

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );

    let options = NativeOptions {
        android_app: Some(android_app),
        ..Default::default()
    };

    eframe::run_native(
        "GitMarket",
        options,
        Box::new(|_cc| Ok(Box::new(app::GitMarketApp::default()))),
    )
    .unwrap();
}
