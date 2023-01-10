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
        println!("Sample data len: {}", sample.data.len());
        println!("Sample reported len: {}", sample.length);
        Self {
            num_sample: 0,
            _sample_rate: sample_rate,
            sample,
        }
    }

    fn get_finetune(&self) -> u8 {
        if (self.sample.finetune&0b1000) != 0 {
            self.sample.finetune|0b11110000
        } else {
            self.sample.finetune
        }
    }

    /// Sets the sample rate while accounting for finetuning.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self._sample_rate = (sample_rate*(1.06_f32.powf((self.get_finetune() as i8 as f32)/8.0))) as u32;
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