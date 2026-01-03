# mdmp4rev

`mdmp4rev` is a robust command-line utility written in Rust that simplifies the process of reversing MP4 video files. It acts as a smart wrapper around **FFmpeg**, automating the complex filter chains required to reverse both video and audio streams seamlessly.

## Features

*   **Simple Interface:** Converts videos with a single command.
*   **Audio & Video:** Reverses both visual and audio tracks (`reverse` + `areverse`).
*   **Smart Naming:** Automatically generates output filenames (e.g., `input.mp4` -> `input-rev.mp4`).
*   **Validation:** Ensures input validity and dependency availability before processing.
*   **Robust Error Handling:** Provides clear, actionable error messages.

## Prerequisites

*   **Rust**: A recent stable version of the Rust toolchain.
*   **FFmpeg**: Must be installed and available in your system's `PATH`.
    *   **Windows**: [Download build](https://ffmpeg.org/download.html)
    *   **macOS**: `brew install ffmpeg`
    *   **Linux**: `sudo apt install ffmpeg`

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone <repository-url>
cd mdmp4rev
cargo build --release
```

The compiled binary will be located in `target/release/mdmp4rev`.

## Usage

Run the tool by providing the path to an MP4 file:

```bash
# Using cargo
cargo run --release -- path/to/video.mp4

# Using the binary directly
./target/release/mdmp4rev path/to/video.mp4
```

### Example

```bash
$ mdmp4rev my_skate_trick.mp4
Successfully created reversed video: "my_skate_trick-rev.mp4"
```

## Development

This project uses standard Rust tooling and "Ports and Adapters" architecture to ensure high testability.

### Architecture
The core logic is encapsulated in the `VideoReverser` struct, which uses a `CommandRunner` trait to abstract system calls. This allows the application logic to be fully tested without requiring FFmpeg to be installed on the test machine.

### Testing

Run the unit test suite (mocks external dependencies):
```bash
cargo test
```

Run integration tests (requires local FFmpeg installation):
```bash
cargo test -- --ignored
```

### Code Quality

Ensure your changes meet the project standards:
```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings
```

## License

[MIT License](LICENSE)
