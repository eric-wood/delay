use std::f32::consts;

#[derive(Copy, Clone, Debug)]
pub enum Type {
  HighPass,
  LowPass,
  BandPass,
  AllPass,
  Notch,
  Peak,
}

pub struct Svf {
  filter_type: Type,
  sample_rate: f32,
  a1: f32,
  a2: f32,
  a3: f32,
  k: f32,

  ic1eq: f32,
  ic2eq: f32,
}

impl Svf {
  pub fn new(cutoff: f32, resonance: f32, filter_type: Type, sample_rate: f32) -> Self {
    let g = (consts::PI * (cutoff / sample_rate)).tan();
    let k = 2.0 - (1.9 * resonance.min(1.0).max(0.0));

    let a1 = 1.0 / (1.0 + (g * (g + k)));
    let a2 = g * a1;
    let a3 = g * a2;

    Svf {
      sample_rate,
      a1,
      a2,
      a3,
      k,
      ic1eq: 0.0,
      ic2eq: 0.0,
      filter_type,
    }
  }

  pub fn set(&mut self, cutoff: f32, resonance: f32) {
    let new = Self::new(cutoff, resonance, self.filter_type, self.sample_rate);

    self.a1 = new.a1;
    self.a2 = new.a2;
    self.a3 = new.a3;
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let v3 = input - self.ic2eq;
    let v1 = (self.a1 * self.ic1eq) + (self.a2 * v3);
    let v2 = self.ic2eq + (self.a2 * self.ic1eq) + (self.a3 * v3);

    self.ic1eq = (2.0 * v1) - self.ic1eq;
    self.ic2eq = (2.0 * v2) - self.ic2eq;

    match self.filter_type {
      Type::LowPass => v2,
      Type::BandPass => v1,
      Type::HighPass => input - self.k * v1 - v2,
      Type::Notch => input - self.k * v1,
      Type::Peak => 2.0 * v2 - input + self.k * v1,
      Type::AllPass => input - 2.0 * self.k * v1,
    }
  }
}
