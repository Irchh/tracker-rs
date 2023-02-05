use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use modfile::ptmf;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::{SamplesConverter, SineWave};
use crate::note::Note;
use crate::sampler::Sample;

pub struct Tracker {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: Vec<Sink>,
    patterns: Vec<Vec<Vec<Option<Note>>>>,
    positions: Vec<u8>,
    samples: Vec<Sample>,
}

impl Tracker {
    pub fn new(tracks: u8, lines: u8) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut _sinks = vec![];
        let mut _tracks = vec![];

        for _ in 0..tracks {
            _sinks.push(Sink::try_new(&stream_handle).unwrap());
            _tracks.push(vec![None; lines as usize])
        }


        Self {
            stream, stream_handle,
            sinks: _sinks, patterns: vec![_tracks.clone()],
            positions: vec![],
            samples: vec![]
        }
    }

    pub fn play(&mut self) {
        for sink in &self.sinks {
            sink.pause();
        }

        //let delay = 60.0/375.0;
        let tpr = 8.0;
        let tempo = 125.0;
        let delay = tpr*2.5/tempo;

        let lines = self.patterns[0][0].len();
        println!("Playing {} lines", lines);
        for &position in &self.positions {
            let pattern = &self.patterns[position as usize];
            for line in 0..lines {
                for (track, notes) in pattern.iter().enumerate() {
                    /*if track != 3 {
                        continue;
                    }*/
                    let note = notes[line].as_ref();
                    if note.is_some() {
                        //println!("Playing note: {}", note.as_ref().unwrap());
                        let note = note.unwrap();
                        let frequency = note.frequency;
                        let sample = note.sample;
                        self.samples[sample as usize].set_sample_rate(frequency);
                        self.sinks[track].append(
                            self.samples[sample as usize].clone()
                                .take_duration(Duration::from_secs_f64(delay))
                                .amplify(0.20)
                        )
                    } else {
                        //println!("Playing note: ---");
                        self.sinks[track].append(
                            SineWave::new(0.0)
                                .take_duration(Duration::from_secs_f64(delay))
                                .amplify(0.20)
                        )
                    }
                }
            }
        }

        for sink in &self.sinks {
            sink.play();
        }

        for sink in &self.sinks {
            sink.sleep_until_end();
        }
    }

    pub fn play_sample(&mut self, sample: u8, frequency: f32) {
        self.sinks[0].pause();

        self.samples[sample as usize].set_sample_rate(frequency);
        self.sinks[0].append(
            self.samples[sample as usize].clone()
                //.take_duration(Duration::from_secs_f64(60.0/375.0))
                .amplify(0.20)
        );

        self.sinks[0].play();
        self.sinks[0].sleep_until_end();
    }

    pub fn load_file(&mut self, file: &str) {
        let mut reader = BufReader::new(File::open(file).unwrap());
        let mut module = ptmf::read_mod(&mut reader, true).unwrap();

        println!("Loading: {}", module.name);
        println!("\tLength: {:?}", module.length);
        //println!("\tPatterns: {:?}", module.patterns);
        println!("\tPositions: {:?}", module.positions);
        println!("\tSamples: {:?}", module.sample_info.len());
        //println!("\tSamples: {:?}", module.sample_info);

        for i in 0..module.length {
            self.positions.push(module.positions.data[i as usize]);
        }

        for sample in module.sample_info {
            self.samples.push(Sample::new(44100, sample.clone()));
        }

        let mut prev_freqs = vec![1.0; module.length as usize];

        for _ in 0..(module.patterns.len()-1) {
            self.patterns.push(self.patterns[0].clone());
        }

        for (patern_index, pattern) in module.patterns.iter().enumerate() {
            for (line, row) in pattern.rows.iter().enumerate() {
                for (track, channel) in row.channels.iter().enumerate() {
                    let frequency = if channel.period == 0 {
                        prev_freqs[track]
                    } else {
                        7093789.2/(2.0*channel.period as f32)
                    };

                    prev_freqs[track] = frequency;

                    println!("{patern_index}, {track}, {line}");

                    self.patterns[patern_index][track][line] = Some(Note {
                        frequency,
                        sample: channel.sample_number,
                        effect: channel.effect
                    });
                }
            }
        }

    }
}