use biquad::Biquad;
use biquad::Coefficients;

pub struct Filter {
  cutoff: f32,
  sample_rate: f32,
  coefficients: Coefficients<f32>,
  filter: biquad::DirectForm1<f32>,
}

impl Filter {
  pub fn new(cutoff: f32, sample_rate: f32) -> Self {
    let coefficients = Coefficients::<f32>::from_params(
      biquad::Type::LowPass,
      biquad::Hertz::<f32>::from_hz(sample_rate).unwrap(),
      biquad::Hertz::<f32>::from_hz(cutoff).unwrap(),
      biquad::Q_BUTTERWORTH_F32,
    )
    .unwrap();

    let filter = biquad::DirectForm1::<f32>::new(coefficients);

    Filter {
      cutoff,
      sample_rate,
      coefficients,
      filter,
    }
  }

  pub fn set(&mut self, cutoff: f32) {
    self.cutoff = cutoff;

    self.coefficients = Coefficients::<f32>::from_params(
      biquad::Type::LowPass,
      biquad::Hertz::<f32>::from_hz(self.sample_rate).unwrap(),
      biquad::Hertz::<f32>::from_hz(cutoff).unwrap(),
      biquad::Q_BUTTERWORTH_F32,
    )
    .unwrap();

    self.filter.update_coefficients(self.coefficients);
  }

  #[inline]
  pub fn process(&mut self, input: f32) -> f32 {
    self.filter.run(input)
  }
}
