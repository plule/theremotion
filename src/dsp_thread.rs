use std::slice;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{SampleFormat, StreamConfig};
use crossbeam_channel::Receiver;
use faust_state::{DspHandle, StateHandle};
use faust_types::FaustDsp;

pub struct ParameterUpdate {
    idx: i32,
    value: f32,
}

impl ParameterUpdate {
    pub fn new(idx: i32, value: f32) -> Self {
        Self { idx, value }
    }
}

/// Run the DSP thread
pub fn run<T>(
    mut dsp: DspHandle<T>,
    mut state: StateHandle,
    parameter_rx: Receiver<ParameterUpdate>,
) -> cpal::Stream
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
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {err}");
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
    if let SampleFormat::F32 = sample_format {
        device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let len = data.len();
                    assert!(len <= buffer_size, "Need buffer size of at least {len}");

                    // Retrieve the parameter updates
                    for parameter in parameter_rx.try_iter() {
                        state.set_param(parameter.idx, parameter.value);
                    }
                    state.send();

                    // Compute the DSP
                    dsp.update_and_compute(len as i32, &buffer_input[..], &mut buffer_output[..]);

                    // Send to audio buffer
                    for (out, dsp_sample) in data.iter_mut().zip(&outputs[0]) {
                        *out = *dsp_sample;
                    }
                },
                err_fn,
            )
            .unwrap()
    } else {
        panic!("only looked as f32 rn");
    }
}
