pub struct Delay {
    delay_line: Vec<f32>,
    index: usize,
    mix: f32,
    feedback: f32,
    last_sample: f32,
    length: f32,
    max_length: usize,
    sample_rate: f32,
}

impl Delay {
    pub fn new(mix: f32, feedback: f32, max_length: f32, length: f32, sample_rate: f32) -> Self {
        let max_samples = (sample_rate * max_length) as usize;
        let length_samples = sample_rate * length;

        Delay {
            delay_line: vec![0.0; max_samples as usize],
            index: 0,
            mix,
            feedback,
            last_sample: 0.0,
            length: length_samples,
            max_length: max_samples,
            sample_rate,
        }
    }

    pub fn set(&mut self, mix: f32, feedback: f32, length: f32) {
        self.mix = mix;
        self.feedback = feedback;
        self.length = self.sample_rate * length;
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let dry = (self.last_sample * self.feedback) + input;

        let delay_integer = self.length as usize;

        // Linear interpolation!
        let sample_1 = self.delay_line[(self.index + delay_integer) % self.max_length];
        let sample_2 = self.delay_line[(self.index + delay_integer + 1) % self.max_length];
        let wet = sample_1 + (sample_2 - sample_1) * self.length.fract();

        let out_sample = (wet * self.mix) + (dry * (1.0 - self.mix));
        self.last_sample = out_sample;
        self.delay_line[self.index] = dry;

        self.index = (self.index + self.max_length - 1) % self.max_length;

        return out_sample;
    }
}
