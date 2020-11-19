pub struct Delay {
    delay_line: Vec<f32>,
    index: usize,
    mix: f32,
    feedback: f32,
    last_sample: f32,
}

impl Delay {
    pub fn new(mix: f32, feedback: f32, length: usize, sample_rate: f32) -> Self {
        Delay {
            delay_line: vec![0.0; (sample_rate / 1000.0 * (length as f32)) as usize],
            index: 0,
            mix,
            feedback,
            last_sample: 0.0,
        }
    }

    pub fn set(&mut self, mix: f32, feedback: f32) {
        self.mix = mix;
        self.feedback = feedback;
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let dry = (self.last_sample * self.feedback) + input;
        let wet = self.delay_line[self.index];

        let out_sample = (wet * self.mix) + (dry * (1.0 - self.mix));
        self.last_sample = out_sample;
        self.delay_line[self.index] = dry;

        self.index += 1;
        if self.index >= self.delay_line.len() {
            self.index = 0;
        }

        return out_sample;
    }
}
