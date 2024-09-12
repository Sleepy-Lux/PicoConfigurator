use std::{fs::File, io::{Read, Write}, path::PathBuf};
use indexmap::IndexMap;
use json::JsonValue;

use crate::{recovery::BACKUP_JSON, util::pretty_json::pretty_json};

use super::Configurator::Configurator;

pub fn load_changes(path: &PathBuf, settings: &mut IndexMap<String, JsonValue>) -> bool {
    let mut ok = true;

    let file_result = File::open(path.clone());
    if file_result.is_err() {
        ok = false;
    } else {
        let mut file = file_result.unwrap();
        let mut data = String::new();
        let _ = file.read_to_string(&mut data);
        let parsed_data: JsonValue = json::parse(&data).unwrap();
        let mut index_map = IndexMap::new();
        for (key, value) in parsed_data.entries() {
            index_map.insert(key.to_string(), value.clone());
        }
        
        unsafe {
            let boxed_str = Box::leak(data.into_boxed_str());
            BACKUP_JSON = boxed_str;
        };
        
        *settings = index_map;

    };
    
    settings.insert_before(0, "Home$".to_owned(), 
        json::parse("{ \"Welcome!\": \"Thank for downloading my simple project. \\nfeel free to mess around!\"}").unwrap()
    );

    return ok;
}

pub fn save_changes(main: &mut Configurator) {
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
}