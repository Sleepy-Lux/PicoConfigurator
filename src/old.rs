use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::env;
use std::sync::Arc;
use eframe::egui::{self, vec2, Button, Checkbox, Color32, Frame, IconData, Label, Layout, RichText, ScrollArea, Slider, TextEdit, Ui};
use eframe::{run_native, App};
use serde_json::{Map, Value};

#[derive(Default)]
struct PicoConfigurator {
    current_page: String,
    bg_color: bool,
    settings: String
}

impl PicoConfigurator {

    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut path = PathBuf::from(env::var("APPDATA").unwrap());
        path.push("Pico Connect");
        path.push("settings.json");
        let file = File::open(path);
        if file.is_err() {
            println!("Failed to find file, {}", file.unwrap_err());
            return Self {
                bg_color: false,
                current_page: "Home".to_string(),
                settings: "0".to_string()
            };
        }
        let mut content = String::new();
        let _ = file.unwrap().read_to_string(&mut content);
        return Self {
            bg_color: false,
            current_page: "Home".to_string(),
            settings: content
        };
    }

    fn iterate_values(&self, ui: &mut Ui, obj: &mut Map<String, Value>, prefix: Option<&str>) {
        obj.iter_mut().for_each(|kv| {
            Frame::default()
                .fill(if self.bg_color { Color32::from_rgb(35, 35, 35) } else { Color32::TRANSPARENT })
                .inner_margin(5.0)
                .show(ui, |ui| {
                    if kv.1.is_object() {
                        self.iterate_values(ui, &mut kv.1.as_object().unwrap().clone(), Option::from(format!("{} / {}", prefix.unwrap(), kv.0.as_str())).as_deref());
                    } else {
                        ui.allocate_space(vec2(ui.available_width(), 0.0));
                        ui.horizontal(|ui| {
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("{} / {}", prefix.unwrap(), kv.0)).size(ui.available_height()/1.15));
    
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(5.0);
    
                                match kv.1 {
                                    Value::String(ref mut s) => {
                                        if ui.add(TextEdit::singleline(s)).changed() {
                                            *kv.1 = Value::String(s.clone());
                                        };
                                    },
                                    Value::Bool(ref mut b) => {
                                        if ui.add(Checkbox::new(b, "Enable")).changed() {
                                            &self.settings.replace(from, to) = Value::Bool(b.clone());
                                        };
                                    },
                                    Value::Number(ref mut n) => {
                                        if ui.add(Slider::new(&mut (n.as_f64().unwrap() as f32), 0.0..=100.0)).changed() {
                                            *kv.1 = Value::Number(serde_json::Number::from_f64(n.as_f64().unwrap()).unwrap());
                                        };
                                    },
                                    _ => {
                                        ui.label("Unsupported type");
                                    }
                                }
                            });
                            self
                        });
                    }
                });
        });
    }
}

impl App for PicoConfigurator {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let data: Value = serde_json::from_str(&self.settings).unwrap();
        if data == "0" {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add_space(5.0);
                ui.vertical_centered(|ui| {
                    ui.add_sized(vec2(500.0, 50.0), 
                        Label::new(RichText::new("Couldnt find PicoConnect Settings JSON.").size(26.0))
                    );

                    ui.add_sized(vec2(400.0, 20.0), 
                        Label::new(RichText::new("Do you have pico connect installed and have you opened it?").size(14.0))
                    );
                });
            });
            return;
        }

        egui::SidePanel::left("Categories").default_width(125.0).min_width(100.0).max_width(150.0).show(ctx, |ui| {
            ui.add_space(10.0);

            ui.add_sized(vec2(ui.available_width(), 30.0),
                Button::new(RichText::new("Home").size(20.0).color(Color32::LIGHT_BLUE)).fill(Color32::TRANSPARENT),
            );

            ui.separator();

            data.clone().as_object().unwrap().keys().for_each(|k| {
                if ui.add_sized(vec2(ui.available_width(), 30.0),
                    Button::new(RichText::new(k).size(16.0)).fill(Color32::TRANSPARENT),
                ).clicked() {
                    self.current_page = k.to_string();
                };
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if &self.current_page.to_lowercase() == "home" {
                return;
            };

            ScrollArea::vertical().show(ui, |ui| {
                let binding = serde_json::from_str::<Value>(&self.settings).unwrap();
                let mut obj = binding.as_object().unwrap().get_key_value(&self.current_page)
                    .unwrap().1.as_object().unwrap().clone();
                let _ = &self.iterate_values(ui, &mut obj, Option::from("Pico"));
            });
        });
        
    }

    
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some(vec2(600.0, 400.0));

    let image_bytes: &[u8] = include_bytes!("../resources/icon.png");
    let image = image::load_from_memory(image_bytes).unwrap();
    let (width, height) = image.to_rgba8().dimensions();
    native_options.viewport.icon = Some(Arc::new(IconData {
        rgba: image.to_rgba8().into_raw(),
        height,
        width,
    }));
    let _ = run_native("Pico Configurator", native_options, Box::new(|cc| Ok(Box::new(PicoConfigurator::new(cc)))));
}