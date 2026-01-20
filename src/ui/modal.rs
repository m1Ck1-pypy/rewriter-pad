use egui::{Id, Modal};

use crate::ui::UiApp;

pub struct CloseModal;

impl CloseModal {
    pub fn new() -> Self {
        Self
    }

    pub fn render_close(&self, ctx: &egui::Context, app: &mut UiApp) {
        let modal = Modal::new(Id::new("modal_close")).show(ctx, |ui| {
            ui.set_width(300.0);
            ui.heading("Close the RewritePad");
            ui.add_space(20.0);
            ui.label("You didn't save the changes. Save your changes?");
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    match app.is_new_file {
                        true => app.save_new_file(),
                        false => app.save_updated_file(),
                    };

                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button("No").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Cancel").clicked() {
                    app.close_modal = false;
                }
            });
        });

        if modal.should_close() {
            app.close_modal = false;
        }
    }

    pub fn render_error(&self, ctx: &egui::Context, app: &mut UiApp) {
        let modal = Modal::new(Id::new("modal_error")).show(ctx, |ui| {
            ui.set_width(300.0);
            ui.heading("Error Opening File");
            ui.add_space(20.0);
            ui.label("An error occurred while opening the file. The file format is not supported.");
            ui.add_space(20.0);
            ui.horizontal(|ui| {
                if ui.button("OK").clicked() {
                    app.error_drop_file = false;
                }
            });
        });

        if modal.should_close() {
            app.error_drop_file = false;
        }
    }
}
