use my_app::MyApp;
fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "My App",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
