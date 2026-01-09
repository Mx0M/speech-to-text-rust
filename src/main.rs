use anyhow::Result;
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::audio_utils::process_chunk;
mod audio_utils;
// mod llm;
#[derive(Parser)]
struct Args {
    /// Sample rate in Hz (e.g., 16000 or 48000)
    #[arg(short = 's', long = "sample", default_value = "16000")]
    sample_rate: u32,

    /// Duration of each audio chunk in seconds
    #[arg(short = 'c', long = "chunks", default_value = "4")]
    chunk_secs: usize,
}

fn apply_gain(samples: &mut [i16], gain: f32) {
    for sample in samples.iter_mut() {
        let amplified = (*sample as f32) * gain;
        *sample = amplified.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let host = cpal::default_host();
    let device = host.default_input_device().expect("No input device");
    let mut config = device.default_input_config()?.config();

    config.sample_rate.0 = args.sample_rate;
    config.channels = 1; // mono input

    let sample_rate = config.sample_rate.0 as usize;
    // print!("{}", sample_rate);
    let chunk_size = sample_rate * args.chunk_secs; // 5 seconds

    let buffer = Arc::new(Mutex::new(Vec::<i16>::new()));
    // let stdout = Arc::new(Mutex::new(io::stdout()));

    let buffer_clone = Arc::clone(&buffer);
    // let stdout_clone = Arc::clone(&stdout);
    let chunk_counter = Arc::new(Mutex::new(0));

    let chunk_counter_clone = Arc::clone(&chunk_counter);

    let stream = device.build_input_stream(
        &config,
        move |data: &[i16], _| {
            let mut buf = buffer_clone.lock().unwrap();
            let mut amplified = data.to_vec();
            //voice detection

            apply_gain(&mut amplified, 1.5); // 1.5x gain

            buf.extend_from_slice(data);

            while buf.len() >= chunk_size {
                let chunk: Vec<i16> = buf.drain(..chunk_size).collect();
                //write
                let mut counter = chunk_counter_clone.lock().unwrap();
                let filename = format!("./chunks/chunk_{}.wav", *counter);

                *counter += 1;
                if *counter == 100 {
                    *counter = 1;
                }

                if let Err(e) = audio_utils::write_wav_chunk(&filename, sample_rate as u32, &chunk)
                {
                    eprintln!("❌ Error writing WAV: {}", e);
                }
            }
        },
        |err| eprintln!("Stream error: {:?}", err),
        None,
    )?;
    let dir_path = "./chunks"; // change this to your target directory
    audio_utils::clear_directory(dir_path)?;
    println!("🎤 Streaming raw audio to stdout...");
    stream.play()?;

    loop {
        std::thread::sleep(Duration::from_millis(2));
        let texts = process_chunk().await?;
        for t in texts {
            println!("{}", t);
        }
    }
}
