use my_app::StateManagementApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "State Management App",
        options,
        Box::new(|cc| Ok(Box::new(StateManagementApp::new(cc)))),
    )
}
