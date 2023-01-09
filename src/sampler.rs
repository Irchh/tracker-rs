use std::time::Duration;
use modfile::ptmf::SampleInfo;
use rodio::Source;

#[derive(Clone, Debug)]
pub struct Sample {
    num_sample: usize,
    _sample_rate: u32,
    sample: SampleInfo,
}

impl Sample {
    pub fn new(sample_rate: u32, sample: SampleInfo) -> Self {
        Self {
            num_sample: 0,
            _sample_rate: sample_rate,
            sample,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self._sample_rate = sample_rate;
        self.num_sample = 0;
    }
}

impl Iterator for Sample {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample.data.len() == 0 {
            return Some(0.0);
        }
        if self.num_sample >= self.sample.data.len() {
            self.num_sample = 0;
        }
        let value = self.sample.data[self.num_sample];
        self.num_sample = self.num_sample.wrapping_add(1);
        Some((value as i8 as f32)/(i8::MAX as f32))
    }
}

impl Source for Sample {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.sample.data.len()-self.num_sample)
    }

    fn channels(&self) -> u16 { 1 }

    fn sample_rate(&self) -> u32 {
        self._sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(0, (self.sample.data.len() * 1_000_000_000 / self._sample_rate as usize) as u32))
    }
}