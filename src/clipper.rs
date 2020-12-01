pub struct Clipper {
  gain: f32,
  compensate: bool,
}

impl Clipper {
  pub fn new(gain: f32, compensate: bool) -> Self {
    Clipper { gain, compensate }
  }

  pub fn set(&mut self, gain: f32) {
    self.gain = gain;
  }

  #[inline]
  pub fn process(&mut self, input: f32) -> f32 {
    let gained = input * self.gain;

    // if gained <= -1.0 {
    //   return -2.0 / 3.0;
    // } else if gained >= 1.0 {
    //   return 2.0 / 3.0;
    // }

    // gained - (gained.powf(3.0) / 3.0)

    let clipped = gained.atan();

    if self.compensate {
      clipped / self.gain
    } else {
      clipped
    }
  }
}
