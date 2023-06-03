use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::File;
use super::data_models::{Song};
use walkdir::WalkDir;

const LIBRARY_PATH: &str = "./data/songs/";
const LIBRARY_FILE: &str = "./data/jsons/library.json";

// Initiates the library as a HashMap containing two Strings
//
// This can be accessed by:
// LIBRARY.lock() -> Result<MutexGuard<HashMap<String, String>>, ()>
// This should only be done in this module and only through wrapper functions otherwise the library might break
lazy_static! {
    static ref LIBRARY: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        let file = File::open(LIBRARY_FILE).unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        m.extend(my_map);
        Mutex::new(m)
    };
}

// The trait each struct has to implement to best interact with the song database
// search looks for entries that contain the query and returns them as a Vec<Song>
// get_song gets a specific entry matching the id and returns it as a Song struct
// The id has to be a name returned by search
// You can search for "" to return all entries
pub trait LibraryApi {
    fn search(&self, query: &str) -> Vec<Song>;
    fn get_song(&self, id: &str) -> Song;
}

pub fn search(query: &str) -> Vec<Song> {
    let map = LIBRARY.lock().unwrap().clone();
    let res = super::generic_functions::search(query, &map);
    let mut vec = Vec::new();
    for entry in res {
        vec.push(Song { name: entry.0, file: entry.1 });
    }
    vec
}

pub fn get_entry(key: &str) -> Song { 
    let ret = super::generic_functions::get_entry(key, &LIBRARY.lock().unwrap());
    Song { name: ret.0, file: ret.1 }
}

// Prints the library HashMap to the console
// Only in dev builds
// Should not be used for prints to the user, use search() instead
#[cfg(debug_assertions)]
pub fn print_lib() {
    let lib = LIBRARY.lock().unwrap();
    println!("Library:");
    for (key, value) in lib.iter() {
        println!("Name: {} Filepath: {}", key, value);
    }
}

pub(crate) fn save() {
    let lib = LIBRARY.lock().unwrap().clone();
    File::create(LIBRARY_FILE).unwrap().write_all(serde_json::to_string(&lib).unwrap().as_bytes()).unwrap();
}

pub(crate) fn validate() -> bool{
    let mut validation = HashMap::new();
    for entry in WalkDir::new(LIBRARY_PATH) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {continue;}
        let path = entry.path();
        let extension = path.extension().unwrap().to_str().unwrap();
        if !super::generic_functions::is_audio_file(extension) {continue;}
        let path_name = String::from(path.file_name().unwrap().to_str().unwrap());
        let name = &path_name[..path_name.len()-4];
        validation.insert(name.to_string(), path.to_str().unwrap().to_string());
    }
    LIBRARY.lock().unwrap().clone() == validation
}