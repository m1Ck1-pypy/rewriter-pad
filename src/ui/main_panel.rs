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
                        .id(text_edit_id)
                        .frame(false)
                        .desired_width(ui.available_size().x)
                        .desired_rows((ui.available_size().y / 20.0).max(5.0) as usize)
                        .font(FontSelection::FontId(FontId::monospace(app.font_size)));

                    let text_field = text_edit_widget.show(ui);
                    
                    let text_field_response = text_field.response;

                    if !ctx.memory(|m| m.has_focus(text_edit_id)) {
                        ctx.memory_mut(|m| m.request_focus(text_edit_id));
                        if let Some(text_cursor_range) = text_field.cursor_range {
                            app.highlight_text = text_cursor_range.slice_str(&app.text).to_owned();
                        }
                    }

                    if text_field_response.changed() {
                        app.is_modified = true;
                    };

                    // text_field_response.context_menu(|ui| {
                    //     ui.set_min_width(150.0);
                    //     ctx.memory_mut(|m| m.request_focus(text_edit_id));
                    //     if ui.button("Copy").clicked() {
                    //         ctx.copy_text(app.highlight_text.clone());
                    //         ui.close();
                    //     }
                    //     if ui.button("Paste").clicked() {
                    //         println!("paste");
                    //         ui.close();
                    //     }
                    //     if ui.button("Cut").clicked() {
                    //         println!("cut");
                    //         ui.close();
                    //     }
                    // });
                });
        });
    }
}
