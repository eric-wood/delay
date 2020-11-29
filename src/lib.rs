#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use serde::{Deserialize, Serialize};

use baseplug::{Plugin, ProcessContext};

mod delay;
use delay::Delay;

mod filter;
use filter::Filter;

mod clipper;
use clipper::Clipper;

baseplug::model! {
    #[derive(Debug, Serialize, Deserialize)]
    struct DelayModel {
        #[model(min = 0.0, max = 1.0)]
        #[parameter(name = "mix", unit = "Generic", gradient = "Linear")]
        mix: f32,

        #[model(min = -90.0, max = 0.0)]
        #[parameter(name = "feedback", unit = "Decibels", gradient = "Power(0.15)")]
        feedback: f32,

        #[model(min = 0.10, max = 1.0, smooth_ms = 50.0)]
        #[parameter(name = "time", unit = "Generic", gradient = "Linear")]
        time: f32,

        #[model(min = 0.0, max = 1.0, smooth_ms = 0.0)]
        #[parameter(name = "freeze", unit = "Generic", gradient = "Linear")]
        freeze: f32,

        #[model(min = 0.0, max = 2.0)]
        #[parameter(name = "tone", unit = "Generic", gradient = "Linear")]
        tone: f32,

        #[model(min = 0.0, max = 1.0)]
        #[parameter(name = "threshold", unit = "Generic", gradient = "Linear")]
        threshold: f32,
    }
}

impl Default for DelayModel {
    fn default() -> Self {
        Self {
            mix: 0.5,
            feedback: 0.2,
            time: 0.5,
            freeze: 0.0,
            tone: 1.0,
            threshold: 1.0,
        }
    }
}

struct DelayPlugin {
    delay_l: Delay,
    delay_r: Delay,
    filter_l: Filter,
    filter_r: Filter,
    clipper_l: Clipper,
    clipper_r: Clipper,
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
            delay_l: Delay::new(0.2, 1.0, model.time, sample_rate, 0.0),
            delay_r: Delay::new(0.2, 1.0, model.time, sample_rate, 0.0),
            filter_l: Filter::new(model.tone, sample_rate),
            filter_r: Filter::new(model.tone, sample_rate),
            clipper_l: Clipper::new(model.threshold),
            clipper_r: Clipper::new(model.threshold),
        }
    }

    #[inline]
    fn process(&mut self, model: &DelayModelProcess, ctx: &mut ProcessContext<Self>) {
        let input = &ctx.inputs[0].buffers;
        let output = &mut ctx.outputs[0].buffers;

        for i in 0..ctx.nframes {
            self.delay_l
                .set(model.feedback[i], model.time[i], model.freeze[i]);
            self.delay_r
                .set(model.feedback[i], model.time[i], model.freeze[i]);

            self.filter_l.set(model.tone[i]);
            self.filter_r.set(model.tone[i]);

            self.clipper_l.set(model.threshold[i]);
            self.clipper_r.set(model.threshold[i]);

            let delay_wet_l = self.delay_l.process(input[0][i]);
            let delay_wet_r = self.delay_r.process(input[0][i]);

            let filtered_delay_l = self.filter_l.process(delay_wet_l);
            let filtered_delay_r = self.filter_l.process(delay_wet_r);

            output[0][i] = self.clipper_l.process(input[0][i]);
            output[1][i] = self.clipper_r.process(input[1][i]);

            // output[0][i] = (filtered_delay_l * model.mix[i]) + (input[0][i] * (1.0 - model.mix[i]));
            // output[1][i] = (filtered_delay_r * model.mix[i]) + (input[1][i] * (1.0 - model.mix[i]));
        }
    }
}

baseplug::vst2!(DelayPlugin, b"tAnE");
