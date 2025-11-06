# 🎯 Omega Focus Rust Screen Recording Challenge

Welcome to the Omega Focus technical assessment for Rust developers! This challenge tests your ability to build a high-performance, cross-platform screen recording application.

---

## 📋 Challenge Overview

Build a **CLI-based screen recording tool** in Rust that can efficiently capture screenshots and full-screen video with audio on both macOS and Windows.

### Timeline
- **Deadline**: November 6, 2024 (1 week)

---

## 🎯 Requirements

### Functional Requirements

1. **Screenshot Capture**
   - On-demand screenshot capture
   - Save as standard image format (PNG/JPEG)
   - User-configurable output path

2. **Full Screen Video Recording**
   - Continuous screen recording
   - Audio capture (system audio and/or microphone)
   - Output format: MP4 or WebM
   - User-configurable settings (resolution, FPS, audio source)

3. **CLI Interface**
   - Simple, intuitive command-line interface
   - Configuration via CLI arguments or config file
   - Start/stop recording controls
   - Status feedback during recording

4. **Cross-Platform Support**
   - Must work on **macOS** and **Windows**
   - Consistent behavior across both platforms

### Performance Requirements

**Critical**: Your solution must meet these performance targets:

- ✅ Record **1080p video at 30 FPS**
- ✅ **CPU usage < 30%** during recording (measured on modern hardware)
- ✅ Minimal memory footprint
- ✅ Efficient video encoding with good quality/size ratio

### Technical Constraints

- **Language**: Rust (latest stable)
- **Starting Point**: Build from scratch
- **Dependencies**: Any Rust crates are allowed
- **Video Format**: MP4 or WebM
- **Audio Format**: AAC, Opus, or similar

---

## 📚 Suggested Resources & Reference Projects

While you must build from scratch, these projects may provide architectural inspiration:

- [screenpipe-recorder](https://github.com/mediar-ai/screenpipe) - High-performance screen capture
- [scrap](https://github.com/quadrupleslap/scrap) - Screen capture library
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O
- [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg) - FFmpeg bindings for Rust

### Recommended Approach

Consider these components for your architecture:
1. **Screen Capture**: Platform-specific APIs (Windows: DXGI, macOS: ScreenCaptureKit/AVFoundation)
2. **Audio Capture**: Cross-platform audio libraries
3. **Video Encoding**: FFmpeg or native codecs
4. **CLI Framework**: clap, structopt, or similar

---

## 📊 Evaluation Criteria

Your submission will be evaluated based on:

### 1. **Performance (40%)**
- Meets the 1080p @ 30fps target
- CPU usage stays below 30%
- Memory efficiency
- File size optimization

### 2. **Code Quality (30%)**
- Clean, idiomatic Rust code
- Proper error handling
- Modular architecture
- Documentation and comments

### 3. **Functionality (20%)**
- All required features implemented
- Works reliably on both platforms
- Handles edge cases gracefully

### 4. **User Experience (10%)**
- Intuitive CLI interface
- Clear error messages
- Good documentation

---

## 🚀 Submission Instructions

### 1. Fork & Develop
```bash
# Fork this repository
# Clone your fork
git clone https://github.com/YOUR_USERNAME/rust-screenrec-challenge.git
cd rust-screenrec-challenge

# Create your solution
cargo build --release
```

### 2. Test Your Solution

Ensure your solution meets all requirements:
- ✅ Compiles on both macOS and Windows
- ✅ Records 1080p @ 30fps with <30% CPU
- ✅ Screenshots work correctly
- ✅ Audio is captured properly
- ✅ Output files are valid and playable

### 3. Create Demo Video

**Required**: Submit a demo showing your application in action:
- **Option A**: Upload via [Omega Focus app](https://focus.inc) (preferred)
- **Option B**: Upload to YouTube and include link

Your demo should show:
1. Building and running the application
2. Taking a screenshot
3. Recording a video with audio
4. Playback of the recorded video
5. Brief CPU usage demonstration

### 4. Submit Pull Request

Create a PR with:
- Your complete source code
- Updated README with:
  - Build instructions
  - Usage examples
  - Architecture overview
  - Performance benchmarks (optional but recommended)
- Demo video link
- Any additional documentation

**PR Title Format**: `[Submission] Your Name - Rust Screen Recorder`

**PR Description Must Include**:
- Demo video link
- Platforms tested on
- Any known limitations
- Benchmark results (if available)

---

## 📁 Project Structure

Your submission should follow this basic structure:

```
rust-screenrec-challenge/
├── Cargo.toml              # Project dependencies
├── Cargo.lock              # Locked dependencies
├── README.md               # Your documentation
├── src/
│   ├── main.rs            # CLI entry point
│   ├── cli.rs             # CLI definition layer
│   ├── error.rs           # Error definition layer
│   ├── validation.rs      # Validation layer
│   ├── screenshot.rs      # Screenshot logic
│   ├── capture/           # Screen capture logic
│   ├── audio/             # Audio capture logic
│   ├── encoder/           # Video encoding
│   ├── lib.rs             # Exposes the modules
│   └── config.rs          # Configuration handling
├── examples/              # Usage examples (optional)
└── tests/                 # Unit tests (recommended)
```

---

## 🛠️ Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- ffmpeg installed and available in PATH
  - macOS: `brew install ffmpeg`
  - Windows: Install from `https://www.gyan.dev/ffmpeg/builds/` and add to PATH
- Platform-specific build tools:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

### Build

```bash
cargo build --release
./target/release/screenrec --help
```

### Example CLI Interface

Your CLI might look something like this:

```bash
# Take a screenshot
screenrec screenshot --output ~/Desktop/screenshot.png

# Record with microphone (defaults to built-in mic :1 on macOS)
screenrec record --output video.mp4 --audio mic --fps 30

# Record with microphone using explicit device
screenrec record --output video.mp4 --audio mic --audio-device ":1" --fps 30
screenrec.exe record --audio mic --audio-device "audio=Microphone Array (Intel® Smart Sound Technology for Digital Microphones)" --fps 30 --resolution 1920X1080 --output .\video_demo.mp4

# Record video with system audio(defaults to blackhole :0)
screenrec record --output ~/Desktop/video.mp4 --audio system --duration 60

# Configure settings
screenrec config --resolution 1920x1080 --fps 30 --codec h264

# View current configuration
screenrec config

# Clear/reset all saved configuration
screenrec config --clear

### Notes on Audio
- All audio capture is handled directly by ffmpeg via platform-native APIs (avfoundation on macOS, dshow on Windows).
- **Microphone input** (`--audio mic`):
  - **macOS**: 
    - Without `--audio-device`: Defaults to built-in microphone (`:1`)
    - With `--audio-device`: Uses the specified device (e.g., `--audio-device ":1"` for built-in mic, `:2` for external mic)
  - **Windows**:
    - Without `--audio-device`: Defaults to default microphone (`audio=Microphone Array (Intel® Smart Sound Technology for Digital Microphones)`)
    - With `--audio-device`: 
      - macOS-style indices are supported: `--audio-device ":1"` converts to `audio=Microphone Array (Intel® Smart Sound Technology for Digital Microphones)`
      - For specific devices, use full device name: `--audio-device "audio=Microphone Array (Intel® Smart Sound Technology for Digital Microphones)"`
      - To list available devices: `ffmpeg -f dshow -list_devices true -i dummy`
- **System audio** (`--audio system`):
  - **macOS**: Requires a loopback device (e.g., BlackHole, Loopback). 
    - First, list available devices: `ffmpeg -f avfoundation -list_devices true -i ""`
    - Defaults to `:0` (typically BlackHole if installed), or specify: `--audio system --audio-device ":0"`
    - **Note**: Without a loopback device, system audio capture may not work. Consider installing BlackHole or Loopback.
  - **Windows**: 
    - Defaults to `virtual-audio-capturer` for system audio
    - macOS-style `:0` is supported and converts to `audio=virtual-audio-capturer`
    - For specific devices, use full device name: `--audio-device "audio=virtual-audio-capturer"`

### A/V Sync
- Audio and video are automatically synchronized by ffmpeg when using the same input source.
- For indefinite recordings (no `--duration`), stop with Ctrl+C; ffmpeg will finalize the file and maintain sync.

### Notes on Performance
- For best results, use release builds (`cargo build --release`).
- On macOS, hardware video encoding uses `h264_videotoolbox`. On Windows, `libx264` is used with fast preset for real-time encoding.
```

---

## 📊 Performance Benchmarks

The following benchmarks were measured on different hardware configurations to demonstrate performance characteristics.

### Test Configurations

**macOS Test System:**
- **Hardware**: Apple M3 iMac (24GB RAM)
- **OS**: macOS Sequoia v15.5   
- **FFmpeg**: Hardware-accelerated via `h264_videotoolbox`

**Windows Test System:**
- **Hardware**: Intel Core i5-10135G7 (8 CPUs, 16GB RAM)
- **OS**: Windows 11
- **FFmpeg**: Software encoding via `libx264` (fast preset)

### Benchmark Results

#### 1080p @ 30 FPS Recording

| Platform | Resolution | FPS | CPU Usage | Memory | File Size (60s) | Audio |
|----------|-----------|-----|-----------|---------|----------------|-------|
| **macOS M3** | 1920x1080 | 30 | 15-20% | ~180MB | ~41MB | Mic |
| **Windows i5** | 1920x1080 | 30 | ~14.8% | ~100MB | ~13MB | Mic |


### Performance Characteristics
#### ✅ Requirements Met

- ✅ **1080p @ 30 FPS**: Achieved on all test systems
- ✅ **CPU Usage < 30%**: Met on macOS and Windows
- ✅ **Efficient Encoding**: Hardware acceleration on macOS, optimized software encoding on Windows
- ✅ **Low Memory Footprint**: < 200MB during recording

#### Performance Notes

**macOS:**
- Hardware-accelerated encoding via `h264_videotoolbox` provides excellent performance
- CPU usage remains consistently below 20% for 1080p @ 30 FPS
- System audio capture via BlackHole has minimal performance impact

**Windows:**
- Software encoding via `libx264` with `fast` preset optimized for real-time
- CPU usage typically below 20% for 1080p @ 30 FPS (meets or approaches target)
- Zero-latency tuning ensures minimal encoding delay
- Hardware encoding (`h264_nvenc`) can be used if available for lower CPU usage

#### File Size Optimization

- **MP4 (H.264)**: ~0.75-0.85 MB/s at 1080p @ 30 FPS
- **WebM (VP9)**: ~0.65-0.75 MB/s at 1080p @ 30 FPS (better compression, slightly higher CPU)
- Audio bitrate: 192kbps (balanced quality/size)

#### Screenshot Performance

- **Capture Time**: < 50ms
- **File Size**: ~5MB (PNG, 4480X2520), ~800KB (JPEG, 4480X2520), ~500KB (PNG, 1920X1080), ~270KB (JPEG, 1920X1080)
- **Memory**: < 50MB during capture

---

## ❓ FAQ

**Q: Can I use existing screen recording libraries?**
A: Yes! You can use any Rust crates. However, you must build the complete application yourself.

**Q: Do I need to support Linux?**
A: No, only macOS and Windows are required. Linux support is a bonus but not required.

**Q: What if I can't achieve <30% CPU usage?**
A: Document your efforts and optimizations. Explain any trade-offs made.

**Q: Can I submit multiple approaches?**
A: Submit your best solution in a single PR. You can include alternative approaches in branches.

**Q: How is CPU usage measured?**
A: We'll test on standard hardware (M1 MacBook Pro, Windows i7/Ryzen 7). Use Activity Monitor/Task Manager for local testing.

**Q: Should I include binary releases?**
A: No, we'll build from source. Focus on clear build instructions.

---

## 📞 Questions & Support

For questions about the challenge:
- **Discord**: Join our [Focus Discord server](https://discord.gg/JGdw52sG) and post in the **P2P Task Marketplace Channel**
- **GitHub Issues**: Open an issue in this repository with the `question` label
- Response time: 24-48 hours

---

## 🏆 Winner Selection

The winning submission will be announced on **November 8, 2024** (2 days after deadline).

Selection process:
1. All PRs meeting minimum requirements will be reviewed
2. Code will be tested on macOS and Windows
3. Performance benchmarks will be run
4. Winner selected based on evaluation criteria
5. Winner will be contacted directly

---

## 🙏 Acknowledgments

This challenge is part of Omega Focus's initiative to build high-performance productivity tools for the agent economy. We're excited to see your creative solutions!

Good luck! 🚀

---

**Omega Labs** | [Website](https://omega.inc) | [Omega Focus](https://focus.inc)
