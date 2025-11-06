# Tests

This directory contains unit and integration tests for the screen recorder application.

## Test Structure

### `validation_tests.rs`
Tests for input validation functions:
- **Resolution validation**: Tests valid/invalid formats, out-of-range values, and edge cases
- **FPS validation**: Tests valid ranges (1-120) and invalid values

**Test Coverage:**
- Valid resolution formats (1920x1080, 3840x2160, etc.)
- Invalid formats (spaces, wrong separators, etc.)
- Out-of-range dimensions (too large or zero)
- Valid FPS values (1-120)
- Invalid FPS values (0, >120)

### `config_tests.rs`
Tests for configuration management:
- **AppConfig structure**: Default values, serialization, deserialization
- **TOML parsing**: Full config, partial config, empty config

**Test Coverage:**
- Default configuration
- TOML serialization/deserialization
- Partial configuration (only some fields set)
- Empty configuration

### `error_tests.rs`
Tests for error types and messages:
- **Error formatting**: All error variants produce correct error messages
- **Error context**: Error messages include relevant information

**Test Coverage:**
- InvalidResolutionFormat
- InvalidFps
- InvalidOutputPath
- InvalidMonitorIndex
- ConfigError
- FfmpegError
- Error Debug formatting

### `integration_tests.rs`
Integration tests for component interactions:
- **CLI types**: AudioSource enum behavior
- **Config operations**: Clone, Debug formatting

**Test Coverage:**
- AudioSource enum values
- AppConfig cloning
- AppConfig Debug output

## Running Tests

### Run all tests:
```bash
cargo test
```

### Run specific test file:
```bash
cargo test --test validation_tests
cargo test --test config_tests
cargo test --test error_tests
cargo test --test integration_tests
```

### Run with output:
```bash
cargo test -- --nocapture
```

### Run specific test:
```bash
cargo test test_validate_resolution_valid
```

## Test Statistics

- **Total Tests**: 21 tests
- **Test Files**: 4 files
- **Coverage Areas**:
  - Input validation (resolution, FPS)
  - Configuration management
  - Error handling
  - Type behavior

## Adding New Tests

When adding new functionality:

1. **Unit tests** go in the appropriate test file:
   - Validation logic → `validation_tests.rs`
   - Config operations → `config_tests.rs`
   - Error types → `error_tests.rs`
   - Integration → `integration_tests.rs`

2. **Test naming**: Use descriptive names like `test_<functionality>_<scenario>`

3. **Test structure**:
   ```rust
   #[test]
   fn test_example() {
       // Arrange
       let input = "test";
       
       // Act
       let result = function(input);
       
       // Assert
       assert!(result.is_ok());
   }
   ```

## Notes

- Tests use the `screenrec` crate name (defined in `Cargo.toml`)
- Some tests require file system access (handled via `tempfile` crate)
- Integration tests are designed to work without actual system access where possible
- All tests should pass on both macOS and Windows

