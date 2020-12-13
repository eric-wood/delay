use std::f32::consts;

pub struct SineOscillator {
  sample_rate: f32,
  current: f32,
  rate: f32,
  max: f32,
}

impl SineOscillator {
  pub fn new(sample_rate: f32, rate: f32) -> Self {
    SineOscillator {
      sample_rate,
      rate: 2.0 * consts::PI * (rate / sample_rate),
      max: 2.0 * consts::PI,
      current: 0.0,
    }
  }

  pub fn set(&mut self, rate: f32) {
    self.rate = self.max * (rate / self.sample_rate);
  }

  #[inline]
  pub fn process(&mut self) -> f32 {
    self.current += self.rate;

    if self.current > self.max {
      self.current -= self.max;
    }

    self.current.sin()
  }
}
