# Coverage Improvement Plan

## Goal
Increase code coverage to >98% by refactoring for testability and introducing mocks for external dependencies.

## Stage 1: Refactor CLI Entry Point
**Objective:** Test argument parsing and top-level error handling.
1.  Extract the body of `main` into a `lib.rs` or keep in `main.rs` but isolated as `pub fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>>`.
2.  Add unit tests for:
    *   No arguments provided (Usage print).
    *   Valid arguments passed to `run`.

## Stage 2: Abstract System Command Execution
**Objective:** Test FFmpeg interaction logic without requiring the binary.
1.  Define a trait `FfmpegCommand` (or `CommandRunner`) that abstracts `Command::new`, `arg`, and `output`.
2.  Implement this trait for `std::process::Command` (Real implementation).
3.  Refactor `VideoReverser` to own a `Box<dyn FfmpegCommand>` (or generic).
4.  Implement a `MockCommand` for tests that captures arguments and returns canned `Output`.

## Stage 3: Cover Core Logic with Mocks
**Objective:** Verify logic paths that were previously unreachable or ignored.
1.  **Test Success Path:** specific check that `reverse_video` constructs the correct arguments (`-i`, `-vf reverse`, etc.) and returns the correct path.
2.  **Test FFmpeg Failure:** Mock a non-zero exit code to verify `VideoError::ProcessingError`.
3.  **Test Missing FFmpeg:** Mock the version check failure to verify `VideoError::FFmpegNotFound` without touching `PATH`.

## Stage 4: Cleanup and Verify
1.  Remove or update the `#[ignore]` integration test.
2.  Run `cargo llvm-cov` to confirm 98% coverage.
