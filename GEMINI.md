# mdmp4rev

## Project Overview

`mdmp4rev` is a command-line utility written in Rust designed to reverse MP4 video files. It processes both the video and audio streams to create a reversed playback version of the input file.

The tool acts as a wrapper around **FFmpeg**, automating the arguments required to apply the reverse filters (`reverse` for video, `areverse` for audio).

**Key Features:**
*   **Simple CLI:** Accepts a single input argument (the MP4 file path).
*   **Automatic Output Naming:** Generates the output filename by appending `-rev` to the input filename (e.g., `video.mp4` -> `video-rev.mp4`).
*   **Validation:** Checks for file existence and correct `.mp4` extension.
*   **Error Handling:** Provides descriptive errors for missing input, invalid formats, or missing FFmpeg installation.

## Architecture

The project consists of a single binary crate.

*   **`src/main.rs`**: Contains the entry point and the core logic.
    *   `VideoReverser`: The main struct responsible for checking dependencies and executing the reversal.
    *   `VideoError`: A custom error enum using `thiserror` to handle various failure states (IO, FFmpeg missing, processing errors).
    *   `tests` module: Contains unit tests for filename generation, validation logic, and mocked environment tests.

**Dependencies:**
*   `thiserror`: For ergonomic error handling.
*   `tempfile` (Dev): For creating temporary files during tests.
*   **System Requirement**: `ffmpeg` must be installed and accessible in the system `PATH`.

## Building and Running

**Prerequisites:**
1.  Install Rust (Cargo).
2.  Install FFmpeg and ensure it is in your system's `PATH`.

**Build:**
```bash
cargo build --release
```

**Run:**
```bash
cargo run -- <path_to_video.mp4>
```

**Example:**
```bash
cargo run -- my_vacation.mp4
# Output: Successfully created reversed video: "my_vacation-rev.mp4"
```

**Testing:**
Run the standard test suite:
```bash
cargo test
```

To run the integration test (which attempts to actually call FFmpeg):
```bash
cargo test -- --ignored
```

## Development Conventions

*   **Error Handling:** Use the `VideoError` enum for any operation that might fail.
*   **Testing:** 
    *   Unit tests should mock filesystem operations where possible (using `tempfile`).
    *   Tests involving external command execution (FFmpeg) are marked `#[ignore]` to avoid failing on systems without FFmpeg or to prevent slow execution during standard checks.
*   **Style:** Follow standard Rust formatting (`cargo fmt`) and idioms (Clippy).
