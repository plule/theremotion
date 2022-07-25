use std::ops::RangeInclusive;
use std::slice;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{SampleFormat, StreamConfig};
use faust_state::{DspHandle, StateHandle};
use faust_types::FaustDsp;

const NOTE: &str = "note";
const VOLUME: &str = "volume";
const CUTOFF_NOTE: &str = "cutoff_note";
const RESONANCE: &str = "res";
const SUPERSAW: &str = "supersaw";
const DETUNE: &str = "detune";

/// DSP controls
#[derive(Default, Debug)]
pub struct Controls {
    /// Midi note, 0-127
    pub note: f32,
    /// Volume, -96-0
    pub volume: f32,
    /// Filter cutoff, -20-20
    pub cutoff_note: f32,
    /// Filter resonnance, 1-30
    pub resonance: f32,
    /// Supersaw volume
    pub supersaw: f32,
    /// Supersaw detune
    pub detune: f32,
}

impl Controls {
    /// Read the current control states from the DSP
    pub fn read(&mut self, state: &mut StateHandle) {
        state.update();
        self.note = *state.get_by_path(NOTE).unwrap();
        self.volume = *state.get_by_path(VOLUME).unwrap();
        self.cutoff_note = *state.get_by_path(CUTOFF_NOTE).unwrap();
        self.resonance = *state.get_by_path(RESONANCE).unwrap();
        self.supersaw = *state.get_by_path(SUPERSAW).unwrap();
        self.detune = *state.get_by_path(DETUNE).unwrap();
    }

    pub fn write(&self, state: &mut StateHandle) {
        state.set_by_path(NOTE, self.note).unwrap();
        state.set_by_path(VOLUME, self.volume).unwrap();
        state.set_by_path(CUTOFF_NOTE, self.cutoff_note).unwrap();
        state.set_by_path(RESONANCE, self.resonance).unwrap();
        state.set_by_path(SUPERSAW, self.supersaw).unwrap();
        state.set_by_path(DETUNE, self.detune).unwrap();
        state.send();
    }

    pub fn note_range() -> RangeInclusive<f32> {
        34.0..=72.0
    }

    pub fn volume_range() -> RangeInclusive<f32> {
        -96.0..=0.0
    }

    pub fn cutoff_range() -> RangeInclusive<f32> {
        -20.0..=60.0
    }

    pub fn resonance_range() -> RangeInclusive<f32> {
        1.0..=30.0
    }

    pub fn supersaw_range() -> RangeInclusive<f32> {
        0.0..=1.0
    }

    pub fn detune_range() -> RangeInclusive<f32> {
        0.0..=0.1
    }
}

pub fn run_dsp<T>(mut dsp: Box<DspHandle<T>>) -> cpal::Stream
where
    T: FaustDsp<T = f32> + 'static + Send,
{
    // Init cpal client
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();
    // no way of knowing the buffer size in advance?
    let buffer_size: usize = 3000;
    // Get number of inputs and ouputs
    let num_inputs = dsp.num_inputs();
    let num_outputs = dsp.num_outputs();
    // Init DSP with a given sample rate
    let sample_rate = config.sample_rate.0;
    dsp.init(sample_rate as i32);
    // Init output buffers
    let inputs: Vec<Vec<f32>> = vec![vec![0_f32; buffer_size]; num_inputs];
    let mut outputs: Vec<Vec<f32>> = vec![vec![0_f32; buffer_size]; num_outputs];
    // Map our Vec<Vec<f32>> to a Vec<&f[32]> to create a buffer for the faust lib
    let buffer_input: Vec<&[f32]> = inputs
        .iter()
        .map(|input| unsafe { slice::from_raw_parts(input.as_ptr(), buffer_size) })
        .collect();
    // Map our Vec<Vec<f32>> to a Vec<&f[32]> to create a buffer for the faust lib
    let mut buffer_output: Vec<&mut [f32]> = outputs
        .iter_mut()
        .map(|output| unsafe { slice::from_raw_parts_mut(output.as_mut_ptr(), buffer_size) })
        .collect();
    let stream = if let SampleFormat::F32 = sample_format {
        device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let len = data.len();
                    assert!(len <= buffer_size, "Need buffer size of at least {}", len);
                    dsp.update_and_compute(len as i32, &buffer_input[..], &mut buffer_output[..]);

                    for (out, dsp_sample) in data.iter_mut().zip(&outputs[0]) {
                        *out = *dsp_sample;
                        //*sample = Sample::from(&0.0);
                    }
                },
                err_fn,
            )
            .unwrap()
    } else {
        panic!("only looked as f32 rn");
    };
    stream
}
