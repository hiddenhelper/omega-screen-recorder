# Evaluation Criteria & Testing Methodology

This document outlines how submissions will be evaluated for the Omega Focus Rust Screen Recording Challenge.

## Scoring Breakdown

### 1. Performance (40 points)

**CPU Usage (20 points)**
- <20% CPU: 20 points
- 20-25% CPU: 15 points
- 25-30% CPU: 10 points
- >30% CPU: 0 points

**Target Performance (10 points)**
- Maintains 1080p @ 30fps consistently: 10 points
- Occasional frame drops: 5 points
- Frequent frame drops: 0 points

**Memory Efficiency (10 points)**
- <500MB RAM: 10 points
- 500MB-1GB RAM: 7 points
- 1-2GB RAM: 4 points
- >2GB RAM: 0 points

### 2. Code Quality (30 points)

**Code Organization (10 points)**
- Clear module structure
- Separation of concerns
- Reusable components
- Idiomatic Rust patterns

**Error Handling (8 points)**
- Comprehensive error types
- Graceful error recovery
- Clear error messages
- No unwrap() in production paths

**Documentation (7 points)**
- Clear README with usage instructions
- Code comments where needed
- API documentation
- Architecture explanation

**Code Style (5 points)**
- Follows Rust conventions
- Passes `cargo fmt`
- Minimal `cargo clippy` warnings
- Consistent naming

### 3. Functionality (20 points)

**Screenshot Capture (5 points)**
- Works reliably
- Configurable output
- Proper image format

**Video Recording (10 points)**
- Smooth recording
- Audio sync
- Configurable settings
- Valid output format

**Cross-Platform (5 points)**
- Works on macOS: 2.5 points
- Works on Windows: 2.5 points

### 4. User Experience (10 points)

**CLI Design (5 points)**
- Intuitive commands
- Clear help text
- Sensible defaults

**Error Messages (3 points)**
- Helpful error messages
- Clear instructions on fix

**Documentation (2 points)**
- Easy to build and run
- Clear usage examples

## Testing Environment

### Hardware Specifications

**macOS Testing:**
- Device: M1 MacBook Pro 14"
- CPU: Apple M1 Pro
- RAM: 16GB
- OS: macOS Sonoma

**Windows Testing:**
- CPU: Intel i7-12700K or AMD Ryzen 7 5800X
- RAM: 16GB
- OS: Windows 11

### Performance Testing Methodology

1. **CPU Usage Measurement**
   - Record 60 seconds of 1080p content at 30fps
   - Measure CPU usage via Activity Monitor (macOS) / Task Manager (Windows)
   - Average CPU % over the recording period
   - Exclude startup/shutdown spikes

2. **Memory Usage Measurement**
   - Peak memory usage during 60-second recording
   - Checked via Activity Monitor / Task Manager

3. **Frame Rate Validation**
   - Analyze output video file
   - Verify consistent 30fps
   - Check for frame drops

4. **Video Quality Check**
   - Visual inspection of output
   - Verify audio sync
   - Check file playability

## Automated Tests

While not required, submissions with the following will score bonus points:

- Unit tests for core functionality
- Integration tests
- Benchmark suite
- CI/CD configuration

## Disqualification Criteria

Submissions will be disqualified if:
- Code doesn't compile
- Plagiarized from other submissions
- Contains malicious code
- Doesn't meet minimum requirements (screenshot + video recording)
- Missing demo video

## Edge Cases & Bonus Points

**Bonus points available for:**
- Multi-monitor support (+3 points)
- Configurable codec options (+2 points)
- Resume recording after pause (+2 points)
- Real-time preview (+3 points)
- Excellent test coverage (+2 points)
- Outstanding documentation (+2 points)

**Common edge cases to handle:**
- No audio device available
- Insufficient disk space
- Permission denied errors
- Monitor disconnection during recording
- High DPI displays
- System sleep/wake during recording

## Review Process

1. **Initial Screening** (automated)
   - Code compiles successfully
   - Basic functionality works
   - Demo video provided

2. **Functionality Review**
   - Test all features
   - Verify cross-platform support
   - Check output quality

3. **Performance Benchmarking**
   - Run standardized tests
   - Measure CPU, memory, FPS
   - Compare against requirements

4. **Code Review**
   - Evaluate code quality
   - Check best practices
   - Review architecture

5. **Final Scoring**
   - Aggregate all scores
   - Review demo video
   - Select winner

## Winner Selection

The submission with the **highest total score** wins.

In case of a tie:
1. Performance score takes priority
2. Code quality as tiebreaker
3. Earlier submission time as final tiebreaker

---

## Questions About Evaluation?

Open an issue with the `evaluation-question` label and we'll clarify!

Good luck! 🎯
