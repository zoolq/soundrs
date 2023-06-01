use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde_json::{ self };
use std::fs::File;
use std::io::Write;

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

pub fn load() {
    let file = File::open("./data/data.json").unwrap();
    let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
    let mut lib = LIBRARY.lock().unwrap();
    lib.clear();
    lib.extend(my_map);
}

pub fn save() {
    let hash = LIBRARY.lock().unwrap().clone();
    File::create("./data/data.json").unwrap().write_all(serde_json::to_string(&hash).unwrap().as_bytes()).unwrap();
}