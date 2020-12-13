// Implements a tone control; 1.0 is unffected, < 1.0 is lowpassed, > 1.0 is highpassed

use crate::svf::{Svf, Type};

pub struct Filter {
  mix: f32,
  hpf: Svf,
  lpf: Svf,
}

impl Filter {
  pub fn new(mix: f32, sample_rate: f32) -> Self {
    let hpf = Svf::new(1_000.0, 0.0, Type::HighPass, sample_rate);
    let lpf = Svf::new(400.0, 0.0, Type::LowPass, sample_rate);

    Filter { mix, hpf, lpf }
  }

  pub fn set(&mut self, mix: f32) {
    self.mix = mix;
  }

  #[inline]
  pub fn process(&mut self, input: f32) -> f32 {
    // let hpf_out = self.hpf.process(input);
    // let lpf_out = self.lpf.process(input);

    // For the classic version with the mid hump...
    // (hpf_out * self.mix) + (lpf_out * (1.0 - self.mix))

    // Improved and completely flat at 50%
    // if self.mix > 1.0 {
    //   (hpf_out * (self.mix - 1.0)) + (input * (1.0 - (self.mix - 1.0)))
    // } else {
    //   (lpf_out * (1.0 - self.mix)) + (input * (1.0 - (1.0 - self.mix)))
    // }

    // LPF < 50%, HPF > 50% with sweep instead of mixing
    if self.mix > 1.0 {
      let cutoff = ((self.mix - 1.0) * 5_000.0) + 400.0;
      self.hpf.set(cutoff, 0.0);
      self.hpf.process(input)
    } else {
      let cutoff = (self.mix * 10_000.0) + 400.0;
      self.lpf.set(cutoff, 0.0);
      self.lpf.process(input)
    }
  }
}
