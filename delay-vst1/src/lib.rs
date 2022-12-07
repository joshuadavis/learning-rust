// Loosely based on https://vaporsoft.net/creating-an-audio-plugin-with-rust-vst/
// Updated for vst 0.3 mostly
// See also: https://github.com/RustAudio/vst-rs/tree/master/examples
// `vst` uses macros, so we'll need to specify that we're using them!
#[macro_use]
extern crate vst;

use std::ffi::c_void;
use std::sync::Arc;
use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::channels::ChannelInfo;
use vst::editor::Editor;
use vst::plugin::{CanDo, HostCallback, Info, Plugin, PluginParameters};

use circular_buffer1::VecCircBuf;

#[derive(Default)]
struct DelayVst {
    host_callback : HostCallback,
    delay_buf : VecCircBuf<f32>,
}

// We're implementing a trait `Plugin` that does all the VST-y stuff for us.
impl Plugin for DelayVst {
    fn get_info(&self) -> Info {
        Info {
            name: "DelayVst".to_string(),

            // Used by hosts to differentiate between plugins.
            // Don't worry much about this now - just fill in a random number.
            unique_id: 1337,

            // Let's set the number of inputs and outputs for stereo operation.
            inputs: 2,
            outputs: 2,

            // For now, fill in the rest of our fields with `Default` info.
            ..Default::default()
        }
    }

    fn new(host: HostCallback) -> Self where Self: Sized {
        return Self { host_callback : host, delay_buf : VecCircBuf::new(1024) };
    }

    // Here is where the bulk of our audio processing code goes.
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

        // 'buffer' contains the streams of input and output samples.
        // 'buffer.zip()' returns an iterator over pairs of input/output buffers.
        for (input_buffer, output_buffer) in buffer.zip() {
            // At this point, the buffers contain samples, so we make parallel iterators
            // through the input/output pairs.
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                // Push the input sample into the circular buffer.
                self.delay_buf.add(*input_sample);
                // Grab the delayed sample.  Blend it in.
                *output_sample = *input_sample;
            }
        }
    }
}

// Make sure you call this, or nothing will happen.
plugin_main!(DelayVst);