use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Note {
    pub frequency: f32,
    pub sample: u8,
    pub effect: u16,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "freq {}, sample: {}, effect: {}", self.frequency, self.sample, self.effect)
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn note_to_frequency() {

    }
}