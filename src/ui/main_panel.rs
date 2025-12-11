use egui::{FontId, FontSelection};

use crate::ui::UiApp;

pub struct MainPanel;

impl MainPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &egui::Context, app: &mut UiApp) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();

            egui::ScrollArea::vertical()
                .id_salt("source")
                .show(ui, |ui| {
                    ui.set_min_height(available_size.y); // Ensure minimum height matches available space

                    let text_edit_id = ui.make_persistent_id("main_text_edit");
                    let text_edit_widget = egui::TextEdit::multiline(&mut app.text)
                        .desired_width(available_size.x) // Use available width
                        .frame(false) // Enable frame for better visibility
                        .id(text_edit_id)
                        .desired_rows((available_size.y / 20.0).max(5.0) as usize) // Dynamic rows based on available height
                        .font(FontSelection::FontId(FontId::monospace(app.font_size)));

                    let text_field = text_edit_widget.show(ui);

                    if !ctx.memory(|m| m.has_focus(text_edit_id)) {
                        ctx.memory_mut(|m| m.request_focus(text_edit_id));
                    }

                    if let Some(text_cursor_range) = text_field.cursor_range {
                        let copy_text = text_cursor_range.slice_str(&app.text);
                        app.copy_text = copy_text.to_string();
                    }

                    if text_field.response.changed() {
                        app.is_modified = true;
                    };

                    egui::Popup::context_menu(&text_field.response)
                        .id(egui::Id::new("context_menu"))
                        .show(|ui| {
                            ui.vertical(|ui| {
                                ui.set_min_width(200.0);
                                ui.button("Click!")
                            })
                        });
                });
        });
    }
}
