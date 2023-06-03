use lazy_static::lazy_static;
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs::File;
use super::data_models::Playlist;
use walkdir::WalkDir;

const PLAYLISTS_PATH: &str = "./data/playlists";
const PLAYLISTS_FILE: &str = "playlists.json";

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
        let file = File::open(PLAYLISTS_FILE).unwrap();
        let my_map: HashMap<String, String> = serde_json::from_reader(file).unwrap();
        m.extend(my_map);
        Mutex::new(m)
    };
}

pub fn search(query: &str) -> Vec<Playlist> {
    let map = PLAYLISTS.lock().unwrap().clone();
    let ret = super::generic_functions::search(query, &map);
    let mut vec = Vec::new();
    for entry in ret {
        vec.push(Playlist { name: entry.0, list: entry.1 });
    }
    vec
}

pub fn get_entry(key: &str) -> Playlist {
    let ret = super::generic_functions::get_entry(key, &PLAYLISTS.lock().unwrap());
    Playlist { name: ret.0, list: ret.1 }
}

// The trait each struct has to implement to best interact with the playlist database
// search looks for entries that contain the query and returns them as a Vec<Playlist>
// get_playlist gets a specific entry matching the id and returns it as a Playlist struct
// The id has to be a name returned by search
// You can search for "" to return all entries
pub trait PlaylistApi {
    fn search(&self, query: &str) -> Vec<Playlist>;
    fn get_playlist(&self, id: &str) -> Playlist;
}

#[cfg(debug_assertions)]
pub fn print_playlists() {
    let lib = PLAYLISTS.lock().unwrap();
    println!("Playlists:");
    for (key, value) in lib.iter() {
        println!("Name: {} Filepath: {}", key, value);
    }
}

pub(crate) fn save() {
    let playlists = PLAYLISTS.lock().unwrap().clone();
    File::create(PLAYLISTS_FILE).unwrap().write_all(serde_json::to_string(&playlists).unwrap().as_bytes()).unwrap();
}

pub(crate) fn validate() -> bool{
    let mut validation = HashMap::new();
    for entry in WalkDir::new(PLAYLISTS_PATH) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {continue;}
        let path = entry.path();
        let extension = path.extension().unwrap().to_str().unwrap();
        if !super::generic_functions::is_audio_file(extension) {continue;}
        let path_name = String::from(path.file_name().unwrap().to_str().unwrap());
        let name = &path_name[..path_name.len()-4];
        validation.insert(name.to_string(), path.to_str().unwrap().to_string());
    }
    PLAYLISTS.lock().unwrap().clone() == validation
}