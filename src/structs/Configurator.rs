use std::{env, path::PathBuf};
use indexmap::IndexMap;
use json::JsonValue;
use eframe::{App, CreationContext};

use crate::modules::gui::render_gui;

use super::Config::load_changes;

pub struct Configurator {
    // status variable
    pub ok: bool,
    status: String,

    // data variables
    pub settings_path: PathBuf,
    pub settings: IndexMap<String, JsonValue>,
    pub current_page: String,
}

impl Configurator {
    pub fn set_status(&mut self, new_status: String) -> &String {
        self.status = new_status;
        return &self.status;
    }
    pub fn get_status(&self) -> &String {
        return &self.status;
    }

    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let mut settings = IndexMap::new();

        let mut path = PathBuf::from(env::var("APPDATA").unwrap());
        path.push("Pico Connect");
        path.push("settings.json");

        // Get config file data
        let ok = load_changes(&path, &mut settings);
        
        Self {
            ok,
            status: "".to_string(),

            settings_path: path,
            settings,
            current_page: "Home$".to_owned(),
        }
    }
}

impl App for Configurator {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        let before_changes = self.settings.clone();

        render_gui(self, ctx);

        if before_changes != self.settings {

        };
    }
}