use std::f32::consts;

pub struct SineOscillator {
  sample_rate: f32,
  current_phase: f32,
  step: f32,
  max: f32,
}

impl SineOscillator {
  pub fn new(sample_rate: f32, rate: f32) -> Self {
    SineOscillator {
      sample_rate,
      step: rate / sample_rate,
      max: 2.0 * consts::PI,
      current_phase: 0.0,
    }
  }

  pub fn set(&mut self, rate: f32) {
    self.step = rate / self.sample_rate;
  }

  #[inline]
  pub fn process(&mut self) -> f32 {
    self.current_phase = (self.current_phase + self.step).fract();

    (self.current_phase * self.max).sin()
  }
}
