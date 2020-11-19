#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use serde::{Deserialize, Serialize};

use baseplug::{Plugin, ProcessContext};

mod delay;
use delay::Delay;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct DelayModel {
        #[model(min = -90.0, max = 0.0)]
        #[parameter(name = "mix", unit = "Decibels", gradient = "Power(0.15)")]
        mix: f32,

        #[model(min = -90.0, max = 0.0)]
        #[parameter(name = "feedback", unit = "Decibels", gradient = "Power(0.15)")]
        feedback: f32
    }
}

impl Default for DelayModel {
    fn default() -> Self {
        Self {
            mix: 0.5,
            feedback: 0.2,
        }
    }
}

struct DelayPlugin {
    delay_l: Delay,
    delay_r: Delay,
}

impl Plugin for DelayPlugin {
    const NAME: &'static str = "a delay plugin";
    const PRODUCT: &'static str = "a delay plugin";
    const VENDOR: &'static str = "Heuristic Industries aka Eric Wood";

    const INPUT_CHANNELS: usize = 2;
    const OUTPUT_CHANNELS: usize = 2;

    type Model = DelayModel;

    #[inline]
    fn new(sample_rate: f32, model: &DelayModel) -> Self {
        Self {
            delay_l: Delay::new(model.mix, 0.2, 500, sample_rate),
            delay_r: Delay::new(model.mix, 0.2, 500, sample_rate),
        }
    }

    #[inline]
    fn process(&mut self, model: &DelayModelProcess, ctx: &mut ProcessContext) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            self.delay_l.set(model.mix[i], model.feedback[i]);
            self.delay_r.set(model.mix[i], model.feedback[i]);

            output[0][i] = self.delay_l.process(input[0][i]);
            output[1][i] = self.delay_l.process(input[1][i]);
        }
    }
}

baseplug::vst2!(DelayPlugin, b"tAnE");
