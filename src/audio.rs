use std::f32::consts::PI;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use anyhow::Context;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const FREQS: [usize; 7] = [440, 494, 523, 587, 659, 698, 784];

pub fn start() -> anyhow::Result<(Arc<AtomicUsize>, cpal::Stream)> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .context("Couldn't open default audio output device")?;
    let config = device.default_output_config()?;

    println!(
        "Audio output with {} and config {:?}\n",
        device.name()?,
        config
    );

    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let mut freq = FrequencyFinder::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::clone(&counter);

    let mut index = 0f32;
    let mut samples_left = 0;
    let mut frequency = 0.0;

    let mut next_value = move || {
        if samples_left == 0 {
            let count = counter.swap(0, Ordering::SeqCst);
            index = 0.0;

            if count == 0 {
                frequency = 0.0;
                samples_left = 440;
            } else {
                crate::cli::print_progress().unwrap();
                samples_left = freq.freq(count);
                frequency = samples_left as f32;
                samples_left *= 10;
            }
        }

        samples_left -= 1;
        index = (index + 1.0) % sample_rate;
        (index * frequency * 2.0 * PI / sample_rate).sin() * 0.5
    };

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _| {
            // println!("Writing {} bytes", data.len());
            for frame in data.chunks_mut(channels) {
                let value = next_value();
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| panic!("{}", err),
    )?;

    stream.play()?;

    Ok((counter2, stream))
}

pub struct FrequencyFinder {
    max: usize,
}

impl FrequencyFinder {
    pub fn new() -> Self {
        FrequencyFinder { max: 0 }
    }

    pub fn freq(&mut self, pkt_size: usize) -> usize {
        if pkt_size > self.max {
            self.max = pkt_size
        }
        let idx = (pkt_size as f32 / self.max as f32 * 6.0).round() as usize;
        FREQS[idx]
    }
}
