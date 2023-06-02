use std::collections::VecDeque;
pub trait QueueTools {
    fn current(&mut self) -> Song;
    fn to_place(&mut self, place: usize);
    fn previous(&mut self);
    fn next(&mut self);
    fn queue(&mut self, song: Song);
    fn clear(&mut self);
}

#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub file: String,
}

#[derive(Debug, Clone)]
pub struct Queue {
    pub current_index: usize,
    pub queue: VecDeque<Song>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            current_index: 0,
            queue: VecDeque::new(),
        }
    }
}

impl QueueTools for Queue {
    fn current(&mut self) -> Song {
        self.queue[self.current_index].clone()
    }

    fn to_place(&mut self, place: usize) {
        if self.queue.len() <= place {
            self.current_index = place;
        }
    }

    fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    fn next(&mut self) {
        if self.current_index == self.queue.len() - 1 {
            self.current_index += 1;
        }
    }

    fn queue(&mut self, song: Song) {
        self.queue.push_back(song);
    }

    fn clear(&mut self) {
        self.queue.clear();
    }
}