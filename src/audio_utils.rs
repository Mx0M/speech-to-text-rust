use anyhow::{Context, Result};
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
// use std::path::Path;
// use crate::llm::send_to_ollama;
use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
/// Write a vector of i16 samples as a mono WAV file.

pub fn write_wav_chunk(
    filename: &str,
    sample_rate: u32,
    samples: &[i16],
) -> Result<(), Box<dyn Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(filename, spec)?;
    for sample in samples {
        writer.write_sample(*sample)?;
    }

    writer.finalize()?;
    Ok(())
}

fn transcribe_with_whisper<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let file_path = file_path.as_ref();
    let output = Command::new("./whisper-bin/whisper-cli")
        .arg("-np")
        .arg("-m")
        // .arg("hi") // or "en" for English
        // .arg("--model")
        .arg("./whisper-bin/models/ggml-base.bin") // adjust model name if needed
        .arg("-f")
        .arg(file_path)
        .arg("-t")
        .arg("8")
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to spawn whisper CLI")?;

    let stdout = output.stdout.context("No stdout from whisper process")?;

    let reader = BufReader::new(stdout);
    let mut transcript = String::new();

    for line in reader.lines() {
        let line = line?;
        transcript.push_str(&line);
        transcript.push('\n');
    }

    Ok(transcript)
}

pub async fn process_chunk() -> Result<Vec<String>> {
    let mut results = Vec::new();

    // loop {
    let entries = fs::read_dir("./chunks")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check file is a WAV chunk
        if path.extension().map(|ext| ext == "wav").unwrap_or(false)
            && path
                .file_name()
                .map(|f| f.to_string_lossy().starts_with("chunk_"))
                .unwrap_or(false)
        {
            let file_path = path;

            match transcribe_with_whisper(&file_path) {
                Ok(text) => {
                    // println!("\n{}", text);
                    fs::remove_file(&file_path)?;
                    results.push(text.clone());
                }
                Err(e) => {
                    eprintln!("❌ Failed to transcribe {}: {}", file_path.display(), e);
                }
            }
        }
    }

    // Wait before scanning again
    thread::sleep(Duration::from_millis(50));
    return Ok(results);
    // }
}
pub fn clear_directory<P: AsRef<Path>>(dir: P) -> Result<()> {
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(&path)?; // Recursively delete subdirectories
        } else {
            fs::remove_file(&path)?; // Delete individual files
        }
    }
    Ok(())
}
