use std::fs::File;
use std::io::BufReader;

use rodio::{ Sink, OutputStream, Decoder };

use crate::db::db::Api;

use super::queue::QueueTools;
use super::queue::Song;
use super::queue::Queue;

pub struct Player {
    pub queue: Queue,
    pub sink: Sink,
}

impl Api for Player {
    fn search(&self, query: &str) -> Vec<Song> {
        crate::db::db::search(query)
    }
    
    fn get_song(&self, id: &str) -> Song {
        
    }
}

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

    pub fn play(&mut self) {
        let raw_source = self.queue.current().file;
        let file = BufReader::new(File::open(raw_source).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }
}