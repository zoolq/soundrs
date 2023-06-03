#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub list: String,
}


#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub file: String,
}