use crate::ui::UiApp;

#[derive(Default)]
pub struct BottomBar;

impl BottomBar {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &egui::Context, app: &mut UiApp) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.columns(2, |column| {
                    egui::ScrollArea::vertical()
                        .id_salt("left_side")
                        .show(&mut column[0], |ui| {
                            ui.label(app.file_path.as_str());
                        });
                    egui::ScrollArea::vertical()
                        .id_salt("right_side")
                        .show(&mut column[1], |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                let modified_title = if app.is_modified {
                                    "File: Not saved"
                                } else {
                                    "File: Saved"
                                };

                                ui.label(format!("{}%", app.scale));
                                ui.label(" | ");
                                ui.label(modified_title);
                                ui.label(" | ");
                                ui.label(format!("Symbols: {}", app.text.chars().count()));
                            });
                        });
                });
            });
    }
}
