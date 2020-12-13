// Meant for modulating a delay time; returns time in seconds to add or subtract to delay

use crate::sine_oscillator::SineOscillator;

pub struct Modulation {
  lfo: SineOscillator,
  depth: f32,
}

impl Modulation {
  pub fn new(sample_rate: f32, rate: f32, depth: f32) -> Self {
    Modulation {
      lfo: SineOscillator::new(sample_rate, rate),
      depth,
    }
  }

  pub fn set(&mut self, rate: f32, depth: f32) {
    self.lfo.set(rate);
    self.depth = depth;
  }

  #[inline]
  pub fn process(&mut self) -> f32 {
    self.lfo.process() * self.depth
  }
}
