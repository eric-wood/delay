pub struct Clipper {
  threshold: f32,
}

impl Clipper {
  pub fn new(threshold: f32) -> Self {
    Clipper { threshold }
  }

  pub fn set(&mut self, threshold: f32) {
    self.threshold = threshold;
  }

  #[inline]
  pub fn process(&mut self, input: f32) -> f32 {
    if input <= -self.threshold {
      return -2.0 / 3.0;
    } else if input >= self.threshold {
      return 2.0 / 3.0;
    }

    input - (input.powf(3.0) / 3.0)
  }
}
