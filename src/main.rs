use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VideoError {
    #[error("FFmpeg is not installed or not accessible")]
    FFmpegNotFound,
    #[error("Invalid input file path: {0}")]
    InvalidInput(String),
    #[error("Failed to process video: {0}")]
    ProcessingError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct VideoReverser;

impl VideoReverser {
    /// Creates a new VideoReverser instance
    pub fn new() -> Self {
        Self {}
    }

    /// Checks if ffmpeg is available on the system
    fn check_ffmpeg(&self) -> Result<(), VideoError> {
        match Command::new("ffmpeg").arg("-version").output() {
            Ok(_) => Ok(()),
            Err(_) => Err(VideoError::FFmpegNotFound),
        }
    }

    /// Generates the output filename by appending "-rev" before the extension
    fn generate_output_filename(&self, input_path: &Path) -> PathBuf {
        let stem = input_path.file_stem().unwrap_or_default();
        let extension = input_path.extension().unwrap_or_default();
        let mut new_name = stem.to_os_string();
        new_name.push("-rev");
        let mut output_path = input_path.with_file_name(new_name);
        output_path.set_extension(extension);
        output_path
    }

    /// Reverses the input MP4 file
    pub fn reverse_video<P: AsRef<Path>>(&self, input_path: P) -> Result<PathBuf, VideoError> {
        let input_path = input_path.as_ref();
        
        // Validate input file
        if !input_path.exists() {
            return Err(VideoError::InvalidInput(
                "Input file does not exist".to_string(),
            ));
        }

        // Check file extension
        if input_path.extension().and_then(|ext| ext.to_str()) != Some("mp4") {
            return Err(VideoError::InvalidInput(
                "Input file must be an MP4".to_string(),
            ));
        }

        // Check if ffmpeg is available
        self.check_ffmpeg()?;

        let output_path = self.generate_output_filename(input_path);

        // Execute ffmpeg command to reverse the video
        let result = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-vf")
            .arg("reverse")
            .arg("-af")
            .arg("areverse")
            .arg("-y") // Overwrite output file if it exists
            .arg(&output_path)
            .output()?;

        if !result.status.success() {
            return Err(VideoError::ProcessingError(
                String::from_utf8_lossy(&result.stderr).to_string(),
            ));
        }

        Ok(output_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_generate_output_filename() {
        let reverser = VideoReverser::new();
        let input = Path::new("test.mp4");
        let output = reverser.generate_output_filename(input);
        assert_eq!(output.to_str().unwrap(), "test-rev.mp4");
    }

    #[test]
    fn test_invalid_input_file() {
        let reverser = VideoReverser::new();
        let result = reverser.reverse_video("nonexistent.mp4");
        assert!(matches!(result, Err(VideoError::InvalidInput(_))));
    }

    #[test]
    fn test_invalid_file_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();

        let reverser = VideoReverser::new();
        let result = reverser.reverse_video(&file_path);
        assert!(matches!(result, Err(VideoError::InvalidInput(_))));
    }

    #[test]
    fn test_ffmpeg_not_found() {
        // This test assumes ffmpeg is not in PATH
        // You might need to temporarily modify PATH for this test
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.mp4");
        fs::write(&file_path, "test content").unwrap();

        let reverser = VideoReverser::new();
        let original_path = std::env::var("PATH").unwrap_or_default();
        
        // Temporarily clear PATH to simulate ffmpeg not being available
        std::env::set_var("PATH", "");
        let result = reverser.reverse_video(&file_path);
        
        // Restore PATH
        std::env::set_var("PATH", original_path);
        
        assert!(matches!(result, Err(VideoError::FFmpegNotFound)));
    }

    // Integration test - requires ffmpeg to be installed
    #[test]
    #[ignore] // Run with `cargo test -- --ignored`
    fn test_successful_video_reverse() {
        let dir = tempdir().unwrap();
        let input_path = dir.path().join("input.mp4");
        
        // Create a dummy MP4 file (not actually valid, just for testing)
        fs::write(&input_path, "dummy mp4 content").unwrap();

        let reverser = VideoReverser::new();
        let result = reverser.reverse_video(&input_path);
        
        assert!(result.is_ok());
        let output_path = result.unwrap();
        assert!(output_path.exists());
        assert_eq!(output_path.file_name().unwrap(), "input-rev.mp4");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <input_mp4_file>", args[0]);
        std::process::exit(1);
    }

    let reverser = VideoReverser::new();
    match reverser.reverse_video(&args[1]) {
        Ok(output_path) => println!("Successfully created reversed video: {:?}", output_path),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
