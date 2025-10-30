# CLI Usage Examples

This document shows example CLI interfaces for your screen recording tool. You're free to design your own interface, but these examples show the expected functionality.

## Basic Commands

### Screenshot Capture

```bash
# Take a screenshot and save to current directory
screenrec screenshot --output screenshot.png

# Screenshot with full path
screenrec screenshot --output ~/Desktop/my-screenshot.png

# Screenshot in JPEG format
screenrec screenshot --output screenshot.jpg --format jpeg
```

### Video Recording

```bash
# Basic video recording (30 second default)
screenrec record --output video.mp4

# Record with specific duration
screenrec record --output demo.mp4 --duration 60

# Record with system audio
screenrec record --output video.mp4 --audio system

# Record with microphone
screenrec record --output video.mp4 --audio mic

# Record with both system audio and microphone
screenrec record --output video.mp4 --audio both
```

## Advanced Options

### Resolution and Frame Rate

```bash
# Specify resolution and FPS
screenrec record --output video.mp4 --resolution 1920x1080 --fps 30

# Record at 60 FPS
screenrec record --output high-fps.mp4 --fps 60

# Lower resolution for performance
screenrec record --output video.mp4 --resolution 1280x720 --fps 30
```

### Video Format Options

```bash
# Output as WebM
screenrec record --output video.webm --codec vp9

# MP4 with H.264
screenrec record --output video.mp4 --codec h264

# Specify quality/bitrate
screenrec record --output video.mp4 --bitrate 5000 --quality high
```

### Configuration

```bash
# Save configuration for future use
screenrec config --resolution 1920x1080 --fps 30 --audio system --format mp4

# View current configuration
screenrec config --show

# Reset to defaults
screenrec config --reset
```

## Interactive Mode

```bash
# Start recording with keyboard controls
screenrec record --interactive

# Controls:
#   Space - Pause/Resume
#   Q     - Stop and save
#   Esc   - Cancel recording
```

## Multi-Monitor Support (Bonus Feature)

```bash
# List available displays
screenrec list-displays

# Record specific display
screenrec record --display 1 --output monitor1.mp4

# Record all displays
screenrec record --display all --output all-monitors.mp4
```

## Status and Information

```bash
# Show recording status
screenrec status

# Show system capabilities
screenrec info

# Check audio devices
screenrec list-audio-devices
```

## Example Workflows

### Quick Screenshot

```bash
screenrec screenshot -o screenshot.png
```

### Meeting Recording

```bash
# Record 60-minute meeting with system audio
screenrec record \
  --output meeting.mp4 \
  --duration 3600 \
  --audio system \
  --resolution 1920x1080 \
  --fps 30
```

### Tutorial Recording

```bash
# Record tutorial with microphone
screenrec record \
  --output tutorial.mp4 \
  --audio mic \
  --fps 30 \
  --quality high
```

### Performance Testing

```bash
# Record with performance monitoring
screenrec record \
  --output test.mp4 \
  --duration 60 \
  --show-stats
```

## Expected Output

### Successful Screenshot

```
✓ Screenshot captured successfully
  Output: /Users/username/Desktop/screenshot.png
  Resolution: 1920x1080
  Size: 2.4 MB
```

### Successful Recording

```
⏺  Recording started...
  Output: video.mp4
  Resolution: 1920x1080
  FPS: 30
  Audio: System audio
  Duration: 60 seconds

⏸  Recording: 00:30 / 01:00 [CPU: 28%] [Memory: 450MB]

✓ Recording saved successfully
  Output: /Users/username/video.mp4
  Duration: 01:00
  Size: 45.2 MB
  Avg CPU: 27%
  Avg FPS: 30.0
```

## Error Handling Examples

### No Audio Device

```
✗ Error: No audio device found
  Try running without --audio flag or check your audio settings
```

### Insufficient Permissions

```
✗ Error: Screen recording permission denied
  Please grant screen recording permission in System Preferences (macOS)
  or Settings > Privacy > Screen Recording (Windows)
```

### Disk Space

```
✗ Error: Insufficient disk space
  Required: ~50 MB for 60 second recording
  Available: 12 MB
```

## Configuration File Example

If you implement configuration files, here's a suggested format:

### `~/.screenrec/config.toml`

```toml
[default]
resolution = "1920x1080"
fps = 30
format = "mp4"
codec = "h264"
audio_source = "system"
output_dir = "~/Desktop"

[performance]
max_cpu_percent = 30
buffer_size = 1024
thread_count = 4

[quality]
bitrate = 5000
preset = "medium"
```

## Tips for Implementation

1. **Clear Help Text**: Implement `--help` for all commands
2. **Sensible Defaults**: 1920x1080, 30fps, system audio
3. **Progress Indicators**: Show real-time stats during recording
4. **Error Messages**: Clear, actionable error messages
5. **Validation**: Check permissions, disk space, etc. before starting
6. **Graceful Shutdown**: Handle Ctrl+C to save partial recordings

## Testing Your CLI

Test these scenarios:
- ✓ All basic commands work
- ✓ Help text is clear and useful
- ✓ Errors are handled gracefully
- ✓ Progress indicators are accurate
- ✓ Output files are valid and playable
- ✓ Performance targets are met

---

Good luck building your CLI! 🚀
