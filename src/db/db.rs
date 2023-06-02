use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::File;
use std::io::Write;

use lazy_static::lazy_static;

use serde_json::{ self };

use walkdir::{ WalkDir };

use crate::queue::queue::Song;

// Initiates the library as a HashMap containing two Strings
// This should be initiated on each startup by the load() function
// If this is not done errors will occur
//
// This can be accessed by:
// LIBRARY.lock() -> Result<MutexGuard<HashMap<String, String>>, ()>
// This should only be done in this module and only through wrapper functions otherwise the library might break
lazy_static! {
    static ref LIBRARY: Mutex<HashMap<String, String>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

// The trait each struct has to implement to best interact with the database
// search looks for entries that contain the query and returns them as a Vec<String>
// get_song gets a specific entry matching the id and returns it as a Song struct
// The id has to be the name returned by search
// You can search for "" to return all entries
pub trait Api {
    fn search(&self, query: &str) -> Vec<Song>;
    fn get_song(&self, id: &str) -> Song;
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
    



// Searches the library HashMap for all entries that contain the query
// Use this for retrieving any amount of entries, try to avoid functions that return all entries
pub fn search(query: &str) -> Vec<Song> {
    let lib = LIBRARY.lock().unwrap();
    let mut result: Vec<Song> = Vec::new();
    for key in lib.keys().filter(|&k| k.to_lowercase().contains(query.to_lowercase().as_str())) {
        let key = key;
        let value = lib.get(key).unwrap();
        result.push(Song{name: key.to_owned(), file: value.to_owned()});
    }
    result.sort_by_key(|name| name.name.to_lowercase());
    result
}

// Extensive loading should only be used when the song data was altered manually
// It loads the library manually instead of pulling it from the saved .json file
// Whenever the .json file fails the validate a extensive load is performed ## Todo!() ##
// 
// Normal load:
// Loads the data from the saved .json file
// This should be the default load
// This is faster but less reliable then extensive load
//
// Extensive load:
// Loads the data from the songs folder
// This is slower then the default load but works when something has been changed manually
// Extensive load is initiated whenever the .json file fails to validate
pub fn load(extensive: bool) {
    let mut lib = LIBRARY.lock().unwrap();
    lib.clear();
    if !extensive {
        let file = File::open("./data/data.json").unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        lib.extend(my_map);
    } 
    else if extensive {
        for entry in WalkDir::new("./data/songs") {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {continue;}
            let path = entry.path();
            let extension = path.extension().unwrap().to_str().unwrap();
            if !is_audio_file(extension) {continue;}
            let path_name = String::from(path.file_name().unwrap().to_str().unwrap());
            let name = &path_name[..path_name.len()-4];
            lib.insert(name.to_string(), path.to_str().unwrap().to_string());
        }
    }
}

// Gets a singe entry
// This function should be used for retrieving a single entry
pub fn get_entry(s: &str) -> Song {
    let lib = LIBRARY.lock().unwrap();
    let file = lib.get(s).unwrap().to_owned();
    let name = s.to_owned();
    Song {name, file}
}

// Saves the library to a .json file
// The json data is located in ./data/data.json
// Might add other save types later
pub fn save() {
    let hash = LIBRARY.lock().unwrap().clone();
    File::create("./data/data.json").unwrap().write_all(serde_json::to_string(&hash).unwrap().as_bytes()).unwrap();
}

// Used to validate wheather a file is an audio file or not
// Currently file types like .ogg and .mp4, which are supported by rodio are not supported
fn is_audio_file(extension: &str) -> bool {
    match extension {
        "mp3" | "wav" | "flac"  | "aac" => true,
        _ => false,
    }
}