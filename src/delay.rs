pub struct Delay {
    delay_line: Vec<f32>,
    index: usize,
    mix: f32,
}

impl Delay {
    pub fn new(mix: f32, length: usize, sample_rate: f32) -> Self {
        Delay {
            delay_line: vec![0.0; (sample_rate / 1000.0 * (length as f32)) as usize],
            index: 0,
            mix,
        }
    }

    pub fn set(&mut self, mix: f32) {
        self.mix = mix;
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let wet = self.delay_line[self.index];
        let out_sample = (wet * self.mix) + (input * (1.0 - self.mix));

        self.delay_line[self.index] = input;

        self.index += 1;
        if self.index >= self.delay_line.len() {
            self.index = 0;
        }

        return out_sample;
    }
}
