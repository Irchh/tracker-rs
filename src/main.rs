mod note;
mod tracker;
mod sampler;

use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::SineWave;
use crate::tracker::Tracker;

fn main() {
    let mut tracker = Tracker::new(4, 64);
    tracker.load_file("./3266CHIP.MOD");
    tracker.play_sample(1, 8287.0);
    //tracker.play();
}
