#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use serde::{Deserialize, Serialize};

use baseplug::{Plugin, ProcessContext};

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct GainModel {
        #[model(min = -90.0, max = 3.0)]
        #[parameter(name = "gain", unit = "Decibels",
            gradient = "Power(0.15)")]
        gain: f32
    }
}

impl Default for GainModel {
    fn default() -> Self {
        Self {
            // "gain" is converted from dB to coefficient in the parameter handling code,
            // so in the model here it's a coeff.
            // -0dB == 1.0
            gain: 1.0,
        }
    }
}

struct Gain {
    delay_line: Vec<f32>,
    index: usize,
}

impl Plugin for Gain {
    const NAME: &'static str = "basic gain plug";
    const PRODUCT: &'static str = "basic gain plug";
    const VENDOR: &'static str = "spicy plugins & co";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = GainModel;

    #[inline]
    fn new(sample_rate: f32, _model: &GainModel) -> Self {
        Self {
            delay_line: vec![0.0; sample_rate as usize],
            index: 0,
        }
    }

    #[inline]
    fn process(&mut self, model: &GainModelProcess, ctx: &mut ProcessContext) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            // output[0][i] = input[0][i] * model.gain[i];
            // output[1][i] = input[1][i] * model.gain[i];

            output[0][i] = self.delay_line[self.index];
            output[1][i] = self.delay_line[self.index];

            self.delay_line[self.index] = input[0][i];

            self.index += 1;
            if self.index >= self.delay_line.len() {
                self.index = 0;
            }
        }
    }
}

baseplug::vst2!(Gain, b"tAnE");
