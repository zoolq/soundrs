use std::fs::File;
use std::io::BufReader;

use rodio::{ Sink, OutputStream, Decoder };

use crate::db::data_models::{ Song, Playlist };
use crate::db::db::{ LibraryApi, PlaylistApi };

use super::queue::QueueTools;
use super::queue::Queue;

trait Audio {
    // Getter functions
    fn volume(&self) -> f32;
    fn speed(&self) -> f32;
    // Setter functions
    fn append(&self, song: Song);
    fn set_volume(&self, volume: f32);
    fn set_speed(&self, speed: f32);
    // Plays if paused, pauses if playing
    fn play(&self);
}

// A single player has to be initiated upon startup
// The player struct controlls the audio playback
// Every interaction with playback or queue should be handeled through the player
pub struct Player {
    pub queue: Queue,
    pub sink: Sink,
}

// The player interacts with the library here
// This should be the only way for the program to access the library
impl LibraryApi for Player {
    // Returns a vector containing all the Songs, whos names contain the query
    fn search(&self, query: &str) -> Vec<Song> {
        crate::db::db::search_library(query)
    }
    
    // Gets a single song by its full name
    fn get_song(&self, id: &str) -> Song {
        crate::db::db::get_entry_libraray(id)
    }
}

// Implements interaction with the queue
// This should bethe only way for the program to access the queue
impl QueueTools for Player {
    fn current(&mut self) -> Song {
        self.queue.current()
    }

    fn to_place(&mut self, place: usize) {
        self.queue.to_place(place);
    }

    fn previous(&mut self) {
        self.queue.previous();
    }

    fn next(&mut self) {
        self.queue.next();
    }

    fn queue(&mut self, song: Song) {
        self.queue.queue(song);
    }

    fn clear(&mut self) {
        self.queue.clear();
    }
}

impl Audio for Player {
    fn play(&self) {
        if !self.sink.empty() && self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    fn append(&self, song: Song) {
        let file = BufReader::new(File::open(song.file).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.sink.sleep_until_end();
    }

    fn set_speed(&self, speed: f32) {
        self.sink.set_speed(speed);
    }

    fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    fn speed(&self) -> f32 {
        self.sink.speed()
    }

    fn volume(&self) -> f32 {
        self.sink.volume()
    }
}

impl Player {
    pub fn new() -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Player {
            queue: Queue::new(),
            sink
        }
    }
}