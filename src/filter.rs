use biquad::Biquad;
use biquad::Coefficients;

// Implements a big muff style tone stack, where a high pass and low pass filter are mixed together

pub struct Filter {
  mix: f32,
  hpf: biquad::DirectForm1<f32>,
  lpf: biquad::DirectForm1<f32>,
}

impl Filter {
  pub fn new(mix: f32, sample_rate: f32) -> Self {
    let hpf_coefficients = Coefficients::<f32>::from_params(
      biquad::Type::HighPass,
      biquad::Hertz::<f32>::from_hz(sample_rate).unwrap(),
      biquad::Hertz::<f32>::from_hz(1000.0).unwrap(),
      biquad::Q_BUTTERWORTH_F32,
    )
    .unwrap();

    let lpf_coefficients = Coefficients::<f32>::from_params(
      biquad::Type::LowPass,
      biquad::Hertz::<f32>::from_hz(sample_rate).unwrap(),
      biquad::Hertz::<f32>::from_hz(1000.0).unwrap(),
      biquad::Q_BUTTERWORTH_F32,
    )
    .unwrap();

    let hpf = biquad::DirectForm1::<f32>::new(hpf_coefficients);
    let lpf = biquad::DirectForm1::<f32>::new(lpf_coefficients);

    Filter { mix, hpf, lpf }
  }

  pub fn set(&mut self, mix: f32) {
    self.mix = mix;
  }

  #[inline]
  pub fn process(&mut self, input: f32) -> f32 {
    let hpf_out = self.hpf.run(input);
    let lpf_out = self.lpf.run(input);

    // For the classic version with the mid hump...
    // (hpf_out * self.mix) + (lpf_out * (1.0 - self.mix))

    // Improved and completely flat at 50%
    if self.mix > 1.0 {
      (hpf_out * (self.mix - 1.0)) + (input * (1.0 - (self.mix - 1.0)))
    } else {
      (lpf_out * (1.0 - self.mix)) + (input * (1.0 - (1.0 - self.mix)))
    }
  }
}
