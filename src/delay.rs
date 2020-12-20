use crate::clipper::Clipper;

pub struct Delay {
    delay_line: Vec<f32>,
    index: usize,
    feedback: f32,
    last_sample: f32,
    length: f32,
    max_length: usize,
    sample_rate: f32,
    freeze: f32,
    clipper: Clipper,
}

impl Delay {
    pub fn new(feedback: f32, max_length: f32, length: f32, sample_rate: f32, freeze: f32) -> Self {
        let max_samples = (sample_rate * max_length) as usize;
        let length_samples = sample_rate * length;

        let clipper = Clipper::new(1.0);

        Delay {
            delay_line: vec![0.0; max_samples as usize],
            index: 0,
            feedback,
            last_sample: 0.0,
            length: length_samples,
            max_length: max_samples,
            sample_rate,
            freeze,
            clipper,
        }
    }

    pub fn set(&mut self, feedback: f32, length: f32, freeze: f32) {
        self.feedback = feedback;
        self.length = self.sample_rate * length;
        self.freeze = freeze;
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let mixed = (self.last_sample * self.feedback) + input;
        let dry = self.clipper.process(mixed);

        let delay_integer = self.length as usize;

        // Linear interpolation!
        // let sample_1 = self.delay_line[(self.index + delay_integer) % self.max_length];
        // let sample_2 = self.delay_line[(self.index + delay_integer - 1) % self.max_length];
        // let wet = sample_1 + (sample_2 - sample_1) * self.length.fract();

        // Hermite interpolation!
        let t = self.index + delay_integer + self.max_length;
        let xm1 = self.delay_line[(t - 1) % self.max_length];
        let x0 = self.delay_line[t % self.max_length];
        let x1 = self.delay_line[(t + 1) % self.max_length];
        let x2 = self.delay_line[(t + 2) % self.max_length];
        let c = (x1 - xm1) * 0.5;
        let v = x0 - x1;
        let w = c + v;
        let a = w + v + (x2 - x0) * 0.5;
        let b_neg = w + a;
        let f = self.length.fract();
        let wet = (((a * f) - b_neg) * f + c) * f + x0;

        self.last_sample = wet;

        if self.freeze < 0.5 {
            self.delay_line[self.index] = dry;
        } else {
            self.delay_line[self.index] = wet;
        }

        self.index = (self.index + self.max_length - 1) % self.max_length;

        wet
    }
}
