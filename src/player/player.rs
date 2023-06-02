use std::sync::Mutex;
use lazy_static::lazy_static;
use rodio::Sink;

lazy_static! {
    static ref SINK: Mutex<Sink> = {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Mutex::new(sink)
    };
}

pub fn queue(f: &str) {
    let sink = SINK.lock().unwrap();
    sink.append(rodio::Decoder::new(std::fs::File::open(f).unwrap()).unwrap());
}

pub fn play() {
    let sink = SINK.lock().unwrap();
    if sink.is_paused() {
        sink.play();
    } else {
        sink.pause();
    }
}
