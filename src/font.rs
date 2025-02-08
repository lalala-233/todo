use eframe::egui;
use eframe::egui::Context;
use font_kit::source::SystemSource;

pub fn set_fonts(ctx: &Context) {
    let system_source = SystemSource::new();
    if let Ok(family) = system_source.select_family_by_name("sans-serif") {
        if let Some(handle) = family.fonts().first().cloned() {
            if let Ok(font) = handle.load() {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    egui::FontData::from_owned(font.copy_font_data().unwrap().to_vec()).into(),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "system_font".to_owned());
                ctx.set_fonts(fonts);
            }
        }
    }
}
