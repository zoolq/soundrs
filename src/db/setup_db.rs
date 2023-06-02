use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::{ self };
use std::fs::{File, DirEntry};
use std::io::Write;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

lazy_static! {
    static ref LIBRARY: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn library_print() {
    let lib = LIBRARY.lock().unwrap();
    println!("{:#?}", lib);
}

pub fn search(s: &str) {
    let lib = LIBRARY.lock().unwrap();
    let mut result: Vec<String> = Vec::new();
    for key in lib.keys().filter(|&k| k.to_lowercase().contains(s.to_lowercase().as_str())) {
        result.push(key.clone());
    }
    result.iter().for_each(|x| println!("{}", x));
}

// Extensive loading should only be used when the song data was altered manually
// It loads the library manually instead of pulling it from the saved .json file
// Whenever the .json file fails the validate a extensive load is performed ## Todo!() ##
pub fn load(extensive: bool) {
    let mut lib = LIBRARY.lock().unwrap();
    lib.clear();
    if !extensive {
        // Loads the data from the saved .json file
        // This should be the default load
        // This is faster but less reliable then extensive load
        let file = File::open("./data/data.json").unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        lib.extend(my_map);
    } else {
        for entry in WalkDir::new("./data/songs") {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let path = entry.path();
                let extension = path.extension().unwrap().to_str().unwrap();
                if is_audio_file(extension) {
                    let path_name = String::from(path.file_name().unwrap().to_str().unwrap());
                    let name = &path_name[..path_name.len()-4];
                    lib.insert(name.to_string(), path.to_str().unwrap().to_string());
                }
            } 
        }
    }
}

pub fn save() {
    let hash = LIBRARY.lock().unwrap().clone();
    File::create("./data/data.json").unwrap().write_all(serde_json::to_string(&hash).unwrap().as_bytes()).unwrap();
}

fn is_audio_file(extension: &str) -> bool {
    match extension {
        "mp3" | "wav" | "flac" | "ogg" | "aac" => true,
        _ => false,
    }
}