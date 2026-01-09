# 🎙️ Speech-to-Text (Rust + Whisper)

A high-performance **speech-to-text CLI tool written in Rust**, powered by **OpenAI Whisper**.  
Designed for fast, offline transcription with configurable sample rates and chunking.

---

## ✨ Features

- 🚀 Fast Rust implementation  
- 🎧 Offline speech recognition using Whisper  
- 🔧 Configurable sample rate  
- 🧩 Audio chunking support for long files  
- 💻 Simple command-line interface  

---

## 📦 Requirements

- **Rust** (stable, latest recommended)
- **Whisper model files** (GGML / GGUF, depending on your setup, download  model ggml-base.bin and put under models inside  whisper-bin)
- Supported OS: Linux / macOS (Windows may work with setup)

---

## 🔧 Installation

### 1️⃣ Clone the repository
git clone https://github.com/your-username/speech-to-text-rust.git
cd speech-to-text-rust
Build the project
cargo build --release


## The binary will be available at:

./target/release/speech-to-text-rust

▶️ Usage

##  Basic command:

./speech-to-text-rust --sample 48000 --chunks 2

Available Options
Flag	Description	Example
--sample	Audio sample rate	48000
--chunks	Number of chunks to split audio	2

##  Example:

./speech-to-text-rust --sample 48000 --chunks 2
