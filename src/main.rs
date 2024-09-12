#![windows_subsystem = "windows"]

use std::sync::Arc;
use eframe::{run_native, NativeOptions};
use eframe::egui::*;
use picoconfigurator::structs::Configurator::Configurator;

fn main() {
    let mut native_options = NativeOptions::default();

    // Window size
    native_options.viewport.inner_size = Some(vec2(600.0, 520.0));

    // Setup icon
    let image_bytes: &[u8] = include_bytes!("../resources/icon.png");
    let image_rgba8 = image::load_from_memory(image_bytes).unwrap().to_rgba8();
    let (width, height) = image_rgba8.dimensions();
    native_options.viewport.icon = Some(Arc::new(IconData {
        rgba: image_rgba8.into_raw(),
        height,
        width,
    }));

    let _ = run_native("Pico Configurator", native_options, Box::new(|cc| Ok(Box::new(Configurator::new(cc)))));
}