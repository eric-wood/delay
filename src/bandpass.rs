use crate::svf;
use crate::svf::Svf;

pub struct Bandpass {
  hpf: Svf,
  lpf: Svf,
  resonance: f32,
}

pub fn lpf_cutoff(cutoff: f32, bandwidth: f32) -> f32 {
  let max = 20_000.0;
  let new = cutoff + bandwidth;
  if new >= max {
    new
  } else {
    max
  }
}

pub fn hpf_cutoff(cutoff: f32, bandwidth: f32) -> f32 {
  let min = 10.0;
  let new = (cutoff - bandwidth).abs();

  if new > min {
    new
  } else {
    min
  }
}

impl Bandpass {
  pub fn new(cutoff: f32, bandwidth: f32, sample_rate: f32) -> Self {
    let resonance = 0.0;

    Bandpass {
      hpf: Svf::new(
        hpf_cutoff(cutoff, bandwidth),
        resonance,
        svf::Type::HighPass,
        sample_rate,
      ),
      lpf: Svf::new(
        lpf_cutoff(cutoff, bandwidth),
        resonance,
        svf::Type::BandPass,
        sample_rate,
      ),
      resonance,
    }
  }

  pub fn set(&mut self, cutoff: f32, bandwidth: f32) {
    self.hpf.set(hpf_cutoff(cutoff, bandwidth), self.resonance);
    self.lpf.set(cutoff, self.resonance);
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let hpf_out = self.hpf.process(input);
    let lpf_out = self.lpf.process(hpf_out);

    lpf_out
  }
}
