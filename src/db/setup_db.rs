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

// Extensive loading should only be used when the song data was altered manually
// It loads the library manually instead of pulling it from the saved .json file
// Whenever the .json file fails the validate a extensive load is performed ## Todo!() ##
pub fn load(extensive: bool) {
    if !extensive {
        // Loads the data from the saved .json file
        // This should be the default load
        // This is faster but less reliable then extensive load
        let file = File::open("./data/data.json").unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        let mut lib = LIBRARY.lock().unwrap();
        lib.clear();
        lib.extend(my_map);
    } else {
        // Implement extensive load
        // Loops through the song directory and adds all the keys and values to the library
        todo!()
    }
}

pub fn save() {
    let hash = LIBRARY.lock().unwrap().clone();
    File::create("./data/data.json").unwrap().write_all(serde_json::to_string(&hash).unwrap().as_bytes()).unwrap();
}