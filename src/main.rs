use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::vec;
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
        }
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
                        .size(ui.available_width()/5.4))
                        .fill(Color32::TRANSPARENT)
                );
                if btn.clicked() {
                    self.current_page = k.clone();
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
                ui.vertical_centered(|ui| {
                    let obj_key = vec![self.current_page.clone()];
                    let obj_data = self.settings.get_mut(obj_key.join("").as_str()).unwrap();
                
                    fn recursive_render(ui: &mut Ui, bg: &mut bool, obj_data: &mut Value, obj_path: Vec<String>) -> Value {
                        let mut updated_obj = Map::new();
                        for (k, v) in obj_data.as_object_mut().unwrap().iter_mut() {
                            let mut new_path = obj_path.clone();
                            new_path.push(k.to_string());
                
                            let mut updated_value = Value::Null;
                            if v.is_object() {
                                updated_value = recursive_render(ui, bg, v, new_path)
                            } else {
                                *bg = !*bg;

                                let (res, painter) = ui.allocate_painter(vec2(ui.available_width(), 26.0), Sense::hover());
                                painter.rect_filled(res.rect, 3.0, 
                                    if *bg { Color32::from_hex("#202020").unwrap() } 
                                        else { Color32::TRANSPARENT }
                                );

                                ui.allocate_ui_at_rect(res.rect, |ui| {
                                    ui.add_space(3.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(5.0);
                                        ui.label(obj_path.join("/") + "/" + k);
                                        
                                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                            ui.add_space(5.0);
                                            match v {
                                                Value::String(ref mut s) => {
                                                    if ui.add(TextEdit::singleline(s)).changed() {
                                                        *v = Value::String(s.clone());
                                                    }
                                                },
                                                Value::Bool(ref mut b) => {
                                                    if ui.add(Checkbox::new(b, "Enable")).changed() {
                                                        *v = Value::Bool(*b);
                                                    }
                                                },
                                                Value::Number(ref mut n) => {
                                                    let mut num = n.as_f64().unwrap() as i32;
                                                    if ui.add(Slider::new(&mut num, 0..=200)).changed() {
                                                        *v = Value::Number(serde_json::Number::from(num));
                                                    }
                                                },
                                                _ => {
                                                    ui.label("Unsupported type");
                                                }
                                            }
                                        });
                                    });
                                    ui.add_space(3.0);
                                    
                                    updated_value = v.clone();
                                });
                            };
                
                            updated_obj.insert(k.to_string(), updated_value);
                        }
                
                        return Value::Object(updated_obj);
                    }
                
                    // Apply the recursive rendering
                    *obj_data = recursive_render(ui, &mut true, obj_data, obj_key);
                });
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