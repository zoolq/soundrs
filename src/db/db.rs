use std::{collections::HashMap, hash::Hash};
use std::sync::Mutex;
use std::fs::File;
use std::io::Write;

use lazy_static::lazy_static;

use serde_json::{ self };

use walkdir::{ WalkDir };

use crate::queue::queue::Song;

// The functions that interact with the LIBRARY are named without any suffixes
// The functions that interact with any other Database are suffixed with _databasename
// 
// Example:
//
// search("test") searches LIBRARY
//
// search_playlists searches PLAYLISTS
//
// Initialize the playlists as a HashMap containing two Strings
// 
// Playlists can have two tags infront of them
// Album -- Shows that the playlist is an album
// Playlist || No Tag -- Shows that the playlist is a custom user generated playlist
lazy_static! {
    static ref PLAYLISTS: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        let file = File::open("./data/jsons/playlists.json").unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        m.extend(my_map);
        Mutex::new(m)
    };
}

// Initiates the library as a HashMap containing two Strings
//
// This can be accessed by:
// LIBRARY.lock() -> Result<MutexGuard<HashMap<String, String>>, ()>
// This should only be done in this module and only through wrapper functions otherwise the library might break
lazy_static! {
    static ref LIBRARY: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        let file = File::open("./data/jsons/library.json").unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        m.extend(my_map);
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
pub fn load() {
    let mut lib = LIBRARY.lock().unwrap();
    lib.clear();
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
    let lib = LIBRARY.lock().unwrap().clone();
    File::create("./data/jsons/library.json").unwrap().write_all(serde_json::to_string(&lib).unwrap().as_bytes()).unwrap();
    let playlists = PLAYLISTS.lock().unwrap().clone();
    File::create("./data/jsons/playlists.json").unwrap().write_all(serde_json::to_string(&playlists).unwrap().as_bytes()).unwrap();
}

// The library launches a validation thread upon startup
// The validation thread validates the current library with the files found in the songs path
// This is launched in the background so that lower-end devices don't have to wait for the library to load
// The Vector contains the boolean values in the following order:
// [0] -> Library
// [1] -> Playlists
pub async fn validate() -> Vec<bool> {
    let mut map= HashMap::new();
    let mut result = Vec::new();
    // Insert any newly created databases here to validate them properly
    map.insert("./data/songs".to_string(),LIBRARY.lock().unwrap().clone());
    map.insert("./data/playlists".to_string(), PLAYLISTS.lock().unwrap().clone());
    for (k, v) in map {
        let mut validation = HashMap::new();
        for entry in WalkDir::new(k) {
            let entry = entry.unwrap();
            if !entry.file_type().is_file() {continue;}
            let path = entry.path();
            let extension = path.extension().unwrap().to_str().unwrap();
            if !is_audio_file(extension) {continue;}
            let path_name = String::from(path.file_name().unwrap().to_str().unwrap());
            let name = &path_name[..path_name.len()-4];
            validation.insert(name.to_string(), path.to_str().unwrap().to_string());
        }
        result.push(v == validation);
    }
    result
}

// Used to validate wheather a file is an audio file or not
// Currently file types like .ogg and .mp4, which are supported by rodio are not supported
fn is_audio_file(extension: &str) -> bool {
    match extension {
        "mp3" | "wav" | "flac"  | "aac" => true,
        _ => false,
    }
}