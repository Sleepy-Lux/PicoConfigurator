use std::{fs::File, io::Write, path::PathBuf, time::{SystemTime, UNIX_EPOCH}, vec};

use eframe::egui::*;
use indexmap::IndexMap;
use json::JsonValue;

use crate::{recovery::{BACKUP_JSON, DEFAULT_JSON}, structs::{Config::load_changes, Configurator::Configurator}, util::pretty_json::pretty_json};

fn write_to_file(path: PathBuf, json: String) {
    let mut file = File::create(path);
    let mut json_obj = JsonValue::new_object();
    let json_object: JsonValue = json::parse(&json).unwrap();
    for (key, value) in json_object.entries() {
        json_obj[key] = value.clone();
    };
    let pretty_str = pretty_json(&json_obj, 2);
    let buf = pretty_str.as_bytes();
    let _ = file.as_mut().unwrap().write_all(buf);
}

pub fn render_gui(main: &mut Configurator, ctx: &Context) {
    SidePanel::left("categories")
    .resizable(false)
    .exact_width(100.0)
    .show(ctx, |ui| {
        ui.add_space(3.0);
        // Create the category buttons from the pico connect settings.json
        for (k, _) in &main.settings {
            let color = k.contains("$");

            let btn = ui.add_sized(vec2(ui.available_width()-6.0, 15.0), 
                Button::new(RichText::new(k.replace("$", ""))
                    .size(ui.available_width()/5.0).color(if color { Color32::BLACK } else { Color32::GRAY }))
                    .fill(if color { Color32::LIGHT_BLUE } else { Color32::TRANSPARENT })
                    .rounding(6.0)
            );
            if btn.clicked() {
                main.current_page = k.clone();
            };

            ui.add_space(2.0);

            if k.ends_with("$") {
                ui.separator();
                continue;   
            };
        };

        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            if ui.add(
                Button::new(RichText::new("Source Code").size(ui.available_width()/8.0).color(Color32::BLACK))
                    .fill(Color32::from_rgb(180, 0, 210))
                    .rounding(6.0)
            ).clicked() {
                let _ = open::that("https://github.com/Sleepy-Lux/PicoConfigurator");
            };
        });
        ui.add_space(2.0);
    });

TopBottomPanel::bottom(Id::new("buttons"))
    .resizable(false)
    .exact_height(50.0)
    .show(ctx, |ui| {
        let height = ui.available_height();
        ui.add_space(2.0); // fix the weird slightly higher then correct middle that egui sets?
        ui.horizontal_centered(|ui| {
            ui.add_space(5.0);

            if ui.add_sized(vec2(100.0, height-20.0), 
                Button::new(RichText::new("Reset All Settings").color(Color32::BLACK))
                    .fill(Color32::RED)
                    .rounding(6.0)
            ).clicked() {
                write_to_file(main.settings_path.clone(), DEFAULT_JSON.to_string());
                load_changes(&main.settings_path.clone(), &mut main.settings);
                main.set_status("Reset All Settings".to_string());
            };

            if ui.add_sized(vec2(80.0, height-20.0), 
                Button::new(RichText::new("Revert Changes").color(Color32::BLACK))
                    .fill(Color32::GOLD)
                    .rounding(6.0)
            ).clicked() {
                unsafe {
                    write_to_file(main.settings_path.clone(), BACKUP_JSON.to_string());
                    load_changes(&main.settings_path.clone(), &mut main.settings);
                    main.set_status("Reverted Changes".to_string());
                };
            };

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add_sized(vec2(80.0, height-20.0), 
                    Button::new(RichText::new("Apply / Save").color(Color32::BLACK))
                        .fill(Color32::GREEN)
                        .rounding(6.0)
                ).clicked() {
                    let mut after_changes = main.settings.clone();
                    after_changes.shift_remove("Home$");
                    let mut file = File::create(main.settings_path.clone());
                    if file.is_err() {
                        return;
                    };
        
                    let mut json_obj = JsonValue::new_object();
                    for (key, value) in after_changes {
                        json_obj[key] = value;
                    };
                    let pretty_str = pretty_json(&json_obj, 2);
        
                    let buf = pretty_str.as_bytes();
                    let _ = file.as_mut().unwrap().write_all(buf);
                    main.set_status("Applied/Saved Changes".to_string());
                    load_changes(&main.settings_path.clone(), &mut main.settings);
                };

                let now: SystemTime = SystemTime::now();
                let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
                let total_seconds = duration_since_epoch.as_secs();
                let total_minutes = total_seconds / 60;
                let hours = (total_minutes / 60) % 24;
                let minutes = total_minutes % 60;
                ui.label(
                    RichText::new(main.get_status().to_string() + " [" + &hours.to_string() + ":" + &minutes.to_string() + "]")
                        .size(10.0)
                );
            });
        });
    });

CentralPanel::default()
    .show(ctx, |ui| {
        if !&main.ok {
            ui.centered_and_justified(|ui| {
                ui.label(
                    RichText::new("Could not load Pico Connect settings file.").size(ui.available_width()/22.0)
                );
            });
            return;
        };

        if main.current_page.to_lowercase().eq("home$") {
            ui.centered_and_justified(|ui| {
                let home = main.settings.get("Home$").unwrap();
                for (_, v) in home.entries() {
                    ui.label(
                        RichText::new(v.as_str().unwrap().to_string()).size(ui.available_width()/22.0)
                    );
                };
            });
            return;
        };

        ui.vertical_centered(|ui| {
            let obj_key = vec![main.current_page.clone()];
            let obj_data = main.settings.get_mut(obj_key.join("").as_str()).unwrap();
            
            // Create inner function to properly recursively call for sub items
            fn recursive_render(ui: &mut Ui, bg: &mut bool, obj_data: &mut JsonValue, obj_path: Vec<String>) -> JsonValue {
                let mut updated_obj = IndexMap::new();
                for (k, v) in obj_data.entries_mut() {
                    let mut new_path = obj_path.clone();
                    new_path.push(k.to_string());
            
                    let mut updated_json_value = JsonValue::Null;
                    if v.is_object() {
                        updated_json_value = recursive_render(ui, bg, v, new_path)
                    } else {
                        // Render the item into the UI with a proper background and spacing
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
                                ui.label(obj_path.join(" / ") + " /");
                                ui.label(RichText::new(k.trim()).color(Color32::LIGHT_BLUE));
                                    
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.add_space(5.0);
                                    match v {
                                        JsonValue::Short(ref mut s) => {
                                            if ui.add(TextEdit::singleline(&mut s.to_string()).desired_width(120.0)).changed() {
                                                *v = JsonValue::Short(s.clone());
                                            }
                                        },
                                        JsonValue::Boolean(ref mut b) => {
                                            if ui.add(Checkbox::new(b, "Enable")).changed() {
                                                *v = JsonValue::Boolean(*b);
                                            }
                                        },
                                        JsonValue::Number(ref mut n) => {
                                            let mut num = n.as_fixed_point_i64(0).unwrap();
                                            if ui.add(Slider::new(&mut num, 0..=200)).changed() {
                                                *v = JsonValue::Number(num.into());
                                            }
                                        },
                                        _ => {
                                            println!("{:?}", v);
                                            ui.label("Unsupported type");
                                        }
                                    }
                                });
                            });
                            ui.add_space(3.0);
                                
                            // These end few bits are just to make sure it properly propagates upwards
                            updated_json_value = v.clone();
                        });
                    };
        
                    updated_obj.insert(k.to_string(), updated_json_value);
                }
            
                return JsonValue::Object(updated_obj.into_iter().collect());
        
            }
        
            // Apply the recursive rendering
            *obj_data = recursive_render(ui, &mut true, obj_data, obj_key);
        });
    });
}