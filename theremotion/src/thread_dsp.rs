use std::slice;

use cpal::traits::{DeviceTrait, HostTrait};
use faust_state::{DspHandle, StateHandle};
use faust_types::FaustDsp;
use std::sync::mpsc::Receiver;

/// Parameter update message
pub struct ParameterUpdate {
    idx: i32,
    value: f32,
}

impl ParameterUpdate {
    /// Creates a new [`ParameterUpdate`].
    pub fn new(idx: i32, value: f32) -> Self {
        Self { idx, value }
    }
}

/// Run the DSP thread
pub fn run<T>(
    dsp: DspHandle<T>,
    state: StateHandle,
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
    let config = device
        .default_output_config()
        .expect("No default output config");

    if config.sample_format() != cpal::SampleFormat::F32 {
        // TODO config selection
        panic!("Only F32 sample rate is supported");
    }

    run_stream(config.into(), dsp, device, parameter_rx, state)
}

fn run_stream<T>(
    config: cpal::StreamConfig,
    mut dsp: DspHandle<T>,
    device: cpal::Device,
    parameter_rx: Receiver<ParameterUpdate>,
    mut state: StateHandle,
) -> cpal::Stream
where
    T: FaustDsp<T = f32> + 'static + Send,
{
    // no way of knowing the buffer size in advance?
    let mut buffer_size: usize = 0;
    // Get number of inputs and ouputs
    let num_inputs = dsp.num_inputs();
    let num_outputs = dsp.num_outputs();
    // Init DSP with a given sample rate
    let sample_rate = config.sample_rate.0;
    dsp.init(sample_rate as i32);
    // Init output buffers
    let mut inputs: Vec<Vec<f32>> = vec![vec![0_f32; buffer_size]; num_inputs];
    let mut outputs: Vec<Vec<f32>> = vec![vec![0_f32; buffer_size]; num_outputs];

    device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Ensure the exchange buffers are large enough
                let len = data.len();
                if len > buffer_size {
                    for input in &mut inputs {
                        input.resize(len, 0.0);
                    }
                    for output in &mut outputs {
                        output.resize(len, 0.0);
                    }
                    buffer_size = len;
                }

                // Retrieve the parameter updates
                for parameter in parameter_rx.try_iter() {
                    state.set_param(parameter.idx, parameter.value);
                }
                state.send();

                // Compute the DSP
                // Map our Vec<Vec<f32>> to a Vec<&f[32]> to create a buffer for the faust lib
                let buffer_input: Vec<&[f32]> = inputs
                    .iter()
                    .map(|input| unsafe { slice::from_raw_parts(input.as_ptr(), buffer_size) })
                    .collect();
                // Map our Vec<Vec<f32>> to a Vec<&f[32]> to create a buffer for the faust lib
                let mut buffer_output: Vec<&mut [f32]> = outputs
                    .iter_mut()
                    .map(|output| unsafe {
                        slice::from_raw_parts_mut(output.as_mut_ptr(), buffer_size)
                    })
                    .collect();
                dsp.update_and_compute(len as i32, &buffer_input[..], &mut buffer_output[..]);
                // Send to audio buffer
                for (out, dsp_sample) in data.iter_mut().zip(&outputs[0]) {
                    *out = *dsp_sample;
                }
            },
            |err| log::error!("an error occurred on the output audio stream: {err}"),
            None,
        )
        .unwrap()
}
