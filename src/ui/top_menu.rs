use crate::ui::UiApp;

#[derive(Debug, Default)]
pub struct TopMenu;

impl TopMenu {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&mut self, ctx: &egui::Context, app: &mut UiApp) {
        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show_separator_line(true)
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            app.is_modified = true;
                            app.is_new_file = true;
                            app.text = String::new();
                        };
                        if ui.add(egui::Button::new("Open")).clicked()
                            && let Some(path) = rfd::FileDialog::new()
                                .add_filter("Text files", &["txt"])
                                .add_filter("All files", &["*"])
                                .pick_file()
                        {
                            match std::fs::read_to_string(&path) {
                                Ok(content) => {
                                    app.text = content;
                                    app.file_path = path.to_string_lossy().to_string();
                                    app.is_new_file = false;
                                    app.is_modified = false;
                                }
                                Err(err) => {
                                    eprintln!("Error reading file: {}", err);
                                }
                            }
                        }

                        if ui.button("Save").clicked() {
                            if app.is_new_file {
                                app.save_new_file();
                            } else {
                                app.save_updated_file();
                            }
                        };
                        if ui.button("Save As...").clicked() {
                            app.save_new_file();
                        };
                        if ui.button("Close").clicked() {
                            std::process::exit(0);
                        };
                    });
                });
            });
    }
}
