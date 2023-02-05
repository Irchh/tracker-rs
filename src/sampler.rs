use std::time::Duration;
use modfile::ptmf::SampleInfo;
use rodio::Source;

#[derive(Clone, Debug)]
pub struct Sample {
    num_sample: usize,
    _sample_rate: u32,
    _sample_rate_multiplier: u32,
    sample: SampleInfo,
    data: Vec<u16>,
}

impl Sample {
    pub fn new(sample_rate: u32, mut sample: SampleInfo) -> Self {
        let mut data: Vec<u16> = vec![];
        for i in 0..sample.data.len() {
            if i % 2 == 1 {
                data.push((sample.data[i-1] as u16) | ((sample.data[i] as u16)<<8))
            } else {
                data.push((sample.data[i] as u16) | ((sample.data[i+1] as u16)<<8))
            }
        }
        println!("Sample data len: {}", sample.data.len());
        println!("Sample reported len: {}", sample.length);
        Self {
            num_sample: 0,
            _sample_rate: sample_rate,
            _sample_rate_multiplier: 256,
            sample,
            data,
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
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            return Some(0);
        }
        let index_1 = self.num_sample/self._sample_rate_multiplier as usize;
        let index_2 = (index_1 + 1) % self.data.len();

        let value_1 = self.data[index_1];
        let value_2 = self.data[index_2];

        let second_index_weight = (self.num_sample % self._sample_rate_multiplier as usize) as f32 / 100.0;
        let first_index_weight = 1.0 - second_index_weight;

        let value = value_1 as f32 * first_index_weight + value_2 as f32 * second_index_weight;

        self.num_sample = self.num_sample.wrapping_add(1);
        if self.num_sample >= self.sample.repeat_length as usize * 2 * self._sample_rate_multiplier as usize {
            self.num_sample = self.sample.repeat_start as usize * 2 * self._sample_rate_multiplier as usize;
        }
        Some(value as u16)
    }
}

impl Source for Sample {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.sample.repeat_length as usize - self.num_sample)
    }

    fn channels(&self) -> u16 { 1 }

    fn sample_rate(&self) -> u32 {
        self._sample_rate*self._sample_rate_multiplier
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(0, (self.sample.repeat_length as usize * 1_000_000_000 / self._sample_rate as usize) as u32))
    }
}