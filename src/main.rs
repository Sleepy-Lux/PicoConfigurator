use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::{env, path::PathBuf};
use serde_json::{Map, Value};
use eframe::{run_native, App, CreationContext, NativeOptions};
use eframe::egui::*;
struct PicoConfigurator {
    // status variable
    ok: bool,

    // data variables
    settings: Map<String, Value>,
    current_page: String,

    // visual variables
    bg: bool
}
impl PicoConfigurator {
    fn new(_cc: &CreationContext<'_>) -> Self {
        let mut ok = true;
        let mut settings = Map::new();


        // Get config file data
        let mut path = PathBuf::from(env::var("APPDATA").unwrap());
        path.push("Pico Connect");
        path.push("settings.json");

        let file = File::open(path);
        if file.is_err() {
            ok = false;
        } else {
            let mut data = String::new();
            let _ = file.unwrap().read_to_string(&mut data);
            settings.append(&mut serde_json::from_str(data.as_str()).unwrap());
        };

        settings.insert("Home$".to_owned(), 
            serde_json::from_str("{ \"Welcome!\": \"Thank for downloading my simple project. \\nfeel free to mess around!\"}").unwrap()
        );

        Self {
            ok,

            settings,
            current_page: "Home$".to_owned(),

            bg: false
        }
    }
    fn render_json_object(&mut self, ui: &mut Ui, obj_path: Vec<String>) {
        let mut obj_data = self.settings.get(&obj_path[0]).unwrap().as_object().unwrap();
        for item in obj_path.iter().skip(1) {
            obj_data = self.settings.get(item).unwrap().as_object().unwrap();
        };

        for (k, v) in obj_data.iter_mut() {
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap(); // Get the available space
            painter.rect_filled(rect, 2.0, 
                if self.bg { Color32::from_hex("#303030").unwrap() } else { Color32::TRANSPARENT });
            
            ui.horizontal(|ui| {
                ui.add_space(2.0);
                ui.label(obj_path);
            });
            ui.add_space(2.0)
        };
    }
}

impl App for PicoConfigurator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("categories")
        .resizable(false)
        .exact_width(100.0)
        .show(ctx, |ui| {
            // Create the category buttons from the pico connect settings.json
            for (k, _) in &self.settings {
                let btn = ui.add_sized(vec2(ui.available_width()-6.0, 15.0), 
                    Button::new(RichText::new(k.replace("$", ""))
                        .size(ui.available_width()/6.0))
                        .fill(Color32::TRANSPARENT)
                );
                if btn.clicked() {
                    self.current_page = k.to_string();
                };

                ui.add_space(2.0);

                if k.ends_with("$") {
                    ui.separator();
                    continue;   
                };
            };

            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add(
                    Button::new(RichText::new("Source Code").size(12.0))
                        .fill(Color32::TRANSPARENT)
                );
            });
            ui.add_space(2.0);
        });

        CentralPanel::default().show(ctx, |ui| {
            if !&self.ok {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new("Could not load Pico Connect settings file.").size(ui.available_width()/12.0)
                    );
                });
                return;
            };

            if self.current_page.to_lowercase().eq("home$") {
                ui.centered_and_justified(|ui| {
                    for (_, v) in self.settings.get("Home$").unwrap().as_object().unwrap().iter() {
                        ui.label(
                            RichText::new(v.as_str().unwrap().to_string()).size(24.0)
                        );
                    };
                });
                return;
            };

            ui.vertical_centered(|ui| {
                let obj_key = self.current_page.clone();
                self.render_json_object(ui, vec![obj_key]);
            });
        });
    }
}

fn main() {
    let mut native_options = NativeOptions::default();

    // Window size
    native_options.viewport.inner_size = Some(vec2(600.0, 400.0));

    // Setup icon
    let image_bytes: &[u8] = include_bytes!("../resources/icon.png");
    let image_rgba8 = image::load_from_memory(image_bytes).unwrap().to_rgba8();
    let (width, height) = image_rgba8.dimensions();
    native_options.viewport.icon = Some(Arc::new(IconData {
        rgba: image_rgba8.into_raw(),
        height,
        width,
    }));

    let _ = run_native("Pico Configurator", native_options, Box::new(|cc| Ok(Box::new(PicoConfigurator::new(cc)))));
}