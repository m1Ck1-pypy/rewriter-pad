use egui::{FontId, FontSelection};

use crate::ui::UiApp;

pub struct MainPanel;

impl MainPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &egui::Context, app: &mut UiApp) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("source")
                .show(ui, |ui| {
                    let text_edit_id = ui.make_persistent_id("main_text_edit");
                    let text_edit_widget = egui::TextEdit::multiline(&mut app.text)
                        .desired_width(f32::INFINITY)
                        .frame(false)
                        .id(text_edit_id)
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
                });
        });
    }
}
