#[derive(Debug)]
pub struct Playlist {
    pub name: String,
    pub list: Vec<Song>
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist { name, list: vec![] }
    }

    pub fn from(name: String, list: Vec<Song>) -> Playlist {
        Playlist { name, list }
    }

    pub fn load(path: String) {
        todo!();
    }

    pub fn add(&mut self, song: Song) {
        self.list.push(song);
    }

    pub fn insert(&mut self, index: usize, song: Song) {
        if index > self.list.len() {
            self.list.insert(index, song);
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }
}


#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub file: String,
}