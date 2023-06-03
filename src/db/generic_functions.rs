use std::collections::HashMap;

// Searches the library HashMap for all entries that contain the query
// Use this for retrieving any amount of entries, try to avoid functions that return all entries
pub(crate) fn search(query: &str, map: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();
    for key in map.keys().filter(|&k| k.to_lowercase().contains(query.to_lowercase().as_str())) {
        let key = key;
        let value = map.get(key).unwrap();
        result.push((key.to_string(), value.to_string()));
    }
    result.sort_by_key(|name| name.name.to_lowercase());
    result
}

// Gets a singe entry
// This function should be used for retrieving a single entry
pub(crate) fn get_entry(s: &str, map: &HashMap<String, String>) -> (String, String) {
    let file = map.get(s).unwrap().to_owned();
    let name = s.to_owned();
    (name, file)
}

// Used to validate wheather a file is an audio file or not
// Currently file types like .ogg and .mp4, which are supported by rodio are not supported
pub(crate) fn is_audio_file(extension: &str) -> bool {
    match extension {
        "mp3" | "wav" | "flac"  | "aac" => true,
        _ => false,
    }
}

// Saves the library to a .json file
// The json data is located in ./data/data.json
// Might add other save types later
pub fn save() {
    super::library_functions::save();
    super::playlists_functions::save();
}

// The library launches a validation thread upon startup
// The validation thread validates the current library with the files found in the songs path
// This is launched in the background so that lower-end devices don't have to wait for the library to load
// The Vector contains the boolean values in the following order:
// [0] -> Library
// [1] -> Playlists
pub async fn validate() -> Vec<bool> {
    let mut res = Vec::new();
    res.push(super::library_functions::validate().await);
    res.push(super::playlists_functions::validate().await);
}