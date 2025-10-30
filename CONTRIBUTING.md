# Contributing Guidelines

Thank you for participating in the Omega Focus Rust Screen Recording Challenge!

## Submission Process

### 1. Fork the Repository
Click the "Fork" button at the top right of this repository.

### 2. Clone Your Fork
```bash
git clone https://github.com/YOUR_USERNAME/rust-screenrec-challenge.git
cd rust-screenrec-challenge
```

### 3. Create Your Solution
Build your screen recording solution following the requirements in the README.

### 4. Test Thoroughly
- Test on both macOS and Windows (if possible)
- Verify performance requirements (<30% CPU for 1080p @ 30fps)
- Ensure all features work correctly
- Test edge cases and error handling

### 5. Document Your Work
Update the README with:
- Build instructions for your solution
- Usage examples
- Architecture decisions
- Known limitations
- Performance benchmarks

### 6. Create Demo Video using focus.inc app
Record a demo showing:
- Building the application
- Screenshot functionality
- Video recording with audio
- Playing back the recorded video
- CPU usage during recording

Upload to:
- **Preferred**: Omega Focus app (https://focus.inc)
- **Alternative**: YouTube (unlisted is fine)

### 7. Submit Pull Request

**PR Title Format:**
```
[Submission] Your Name - Rust Screen Recorder
```

**PR Description Template:**
```markdown
## Submission Details

**Name:** Your Name
**Demo Video:** [Link here]
**Tested On:** 
- macOS [version]
- Windows [version]

## Implementation Highlights

[Brief overview of your approach]

## Performance Results Screnshot

- Resolution: 1080p
- FPS: 30
- CPU Usage: [X%]
- Memory Usage: [X MB]
- Test Hardware: [Your specs]

## Known Limitations

[Any limitations or trade-offs]

## Additional Notes

[Anything else we should know]
```

## Code Quality Standards

- **Rust Style**: Follow standard Rust formatting (`cargo fmt`)
- **Linting**: Address clippy warnings (`cargo clippy`)
- **Error Handling**: Use proper Result types, avoid unwrap() in production code
- **Documentation**: Add doc comments for public APIs
- **Testing**: Unit tests are encouraged but not required

## Evaluation Timeline

- **Submission Deadline**: November 4, 2024, 11:59 PM (PST)
- **Review Period**: November 5-7, 2024
- **Winner Announcement**: November 8, 2024

## Questions?

If you have questions about the challenge:
1. Check the FAQ in the main README
2. Open an issue with the `question` label
3. We'll respond within 24-48 hours

## Code of Conduct

- Be respectful in all communications
- Your submission must be your own work
- Don't copy solutions from other participants
- Proper attribution if using code snippets from open source (within reason)

## License

By submitting a PR, you agree that your code will be licensed under the MIT License.

---

Good luck and happy coding! 🚀
