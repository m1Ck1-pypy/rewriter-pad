mod bottom_bar;
mod main_panel;
mod modal;
mod top_menu;

use egui::{IconData, ViewportBuilder};
use std::{
    env, f32,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::ui::{
    bottom_bar::BottomBar, main_panel::MainPanel, modal::CloseModal, top_menu::TopMenu,
};

#[derive(Debug, Default)]
pub struct AppStore {
    open_file_path: String,
}

impl AppStore {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();

        let open_file_path = if args.len() > 1 {
            args[1].clone()
        } else {
            String::new()
        };
        Self { open_file_path }
    }
}

pub type StoreSafe = Arc<Mutex<AppStore>>;

const TITLE: &str = "RewritePad";

pub struct UiApp {
    pub store: StoreSafe,
    pub file_path: String,
    pub text: String,
    pub is_modified: bool,
    pub font_size: f32,
    // pub copy_text: String,
    pub highlight_text: String,
    pub is_new_file: bool,
    pub close_modal: bool,
    pub scale: i32,
}

impl UiApp {
    pub fn new() -> Self {
        let store = Arc::new(Mutex::new(AppStore::new()));

        let file_path = store.lock().unwrap().open_file_path.clone();
        let text = if !file_path.is_empty() {
            std::fs::read_to_string(&file_path).unwrap_or_else(|_| {
                eprintln!(
                    "Warning: Could not read file '{file_path}', starting with empty content",
                );
                String::new()
            })
        } else {
            String::new()
        };
        let is_new_file = file_path.is_empty() || matches!(text.len(), 0);

        Self {
            store,
            file_path,
            text,
            is_new_file,
            is_modified: false,
            font_size: 16.0,
            // copy_text: String::new(),
            highlight_text: String::new(),
            close_modal: false,
            scale: 100,
        }
    }

    /// Изменяем размер шрифта
    fn handle_inputs(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|i| {
            for event in i.events.iter() {
                if i.modifiers.ctrl
                    && let egui::Event::MouseWheel { delta, .. } = event
                    && delta.y != 0.0
                {
                    let zoom_factor = if delta.y > 0.0 { 1.1 } else { 0.9 };
                    self.font_size = (self.font_size * zoom_factor).clamp(8.0, 72.0);
                    self.scale = (self.scale as f32 * zoom_factor).clamp(20.0, 450.0) as i32;
                }
            }

            if i.consume_key(egui::Modifiers::CTRL, egui::Key::S) {
                if self.is_new_file {
                    self.save_new_file();
                } else {
                    self.save_updated_file();
                }
                self.is_new_file = false;
            }
        });
    }

    /// Сохраняем новый файл
    pub fn save_new_file(&mut self) {
        let dir_path = rfd::FileDialog::new()
            .set_file_name("new-text-file")
            .add_filter("Text File", &["txt"])
            .save_file();
        if let Some(dir_path) = dir_path {
            match File::create(&dir_path) {
                Ok(new_file) => {
                    let mut writer = std::io::BufWriter::new(new_file);
                    if let Err(e) = writer.write_all(self.text.as_bytes()) {
                        eprintln!("Error writing to file: {e}");
                    } else {
                        self.file_path = dir_path.to_string_lossy().to_string();
                        self.is_modified = false;
                    }
                }
                Err(err) => {
                    eprintln!("Error creating new file: {err}");
                }
            }
        }
    }

    /// Сохраняем обновленный файл
    pub fn save_updated_file(&mut self) {
        if self.file_path.is_empty() {
            self.save_new_file();
        } else {
            match File::create(&self.file_path) {
                Ok(new_file) => {
                    let mut writer = std::io::BufWriter::new(new_file);
                    if let Err(e) = writer.write_all(self.text.as_bytes()) {
                        eprintln!("Error writing to file: {e}");
                    } else {
                        self.is_modified = false;
                    }
                }
                Err(err) => {
                    eprintln!("Error creating new file: {err}");
                }
            }
        }
    }
}

#[macro_export]
macro_rules! ui_frame {
    ($ctx:expr, $frame:expr, $content:expr) => {
        egui::CentralPanel::default()
            .frame($frame)
            .show($ctx, $content)
    };
}

impl eframe::App for UiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut top_menu = TopMenu::new();
        let mut main_panel = MainPanel::new();
        let mut bottom_bar = BottomBar::new();
        let close_modal = CloseModal::new();

        self.handle_inputs(ctx);

        let close_requested = ctx.input(|i| i.viewport().close_requested());

        if close_requested && self.is_modified {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.close_modal = true;
        }

        ui_frame!(ctx, FRAME, |_ui| {
            top_menu.render(ctx, self);
            main_panel.render(ctx, self);
            bottom_bar.render(ctx, self);

            if self.close_modal {
                close_modal.render(ctx, self);
            }
        });
    }
}

const FRAME: egui::Frame = egui::Frame {
    inner_margin: egui::Margin::same(0),
    outer_margin: egui::Margin::same(0),
    corner_radius: egui::CornerRadius::same(0),
    shadow: egui::Shadow::NONE,
    stroke: egui::Stroke::NONE,
    fill: egui::Color32::from_rgb(255, 255, 255),
};

pub fn app_ui() -> eframe::Result<()> {
    let icon_data = {
        let (icon_rgba, icon_width, icon_height) = {
            let exe_dir = std::env::current_exe()
                .ok()
                .and_then(|path| path.parent().map(|p| p.to_path_buf()))
                .unwrap_or_default();

            let icon_paths = [exe_dir.join("icon.ico"), PathBuf::from("icon.ico")];
            let mut image_result = None;

            for path in &icon_paths {
                if let Ok(image) = image::open(path) {
                    image_result = Some(image.into_rgba8());
                    break;
                }
            }

            if let Some(image) = image_result {
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                (rgba, width, height)
            } else {
                (Vec::new(), 0, 0)
            }
        };

        if icon_width > 0 && icon_height > 0 {
            IconData {
                rgba: icon_rgba,
                width: icon_width,
                height: icon_height,
            }
        } else {
            IconData {
                rgba: vec![0; 4],
                width: 1,
                height: 1,
            }
        }
    };

    let opt = eframe::NativeOptions {
        viewport: ViewportBuilder {
            title: Some(TITLE.to_string()),
            icon: Some(Arc::new(icon_data)),
            inner_size: Some(egui::Vec2::new(1000.0, 560.0)),
            ..Default::default()
        },
        centered: true,
        ..Default::default()
    };

    eframe::run_native("RewritePad", opt, Box::new(|_| Ok(Box::new(UiApp::new()))))
}
