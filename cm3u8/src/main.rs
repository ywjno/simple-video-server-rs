use std::fs;
use std::path::Path;
use std::process::Command;

use clap::Parser;
use rand::Rng;

#[derive(Parser)]
#[command(name = "cm3u8")]
#[command(about = "Convert video files to m3u8 using FFmpeg")]
#[command(version = "1.0")]
struct Args {
    /// Input video file path
    #[arg(short = 'i', long = "input", help = "Path to the input video file")]
    input: String,
}

fn generate_video_id() -> String {
    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnpqrstuvwxyz23456789";
    let mut rng = rand::rng();

    (0..11)
        .map(|_| {
            let idx = rng.random_range(0..CHARS.len());
            CHARS[idx] as char
        })
        .collect()
}

fn convert(input: &str, video_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check if input file exists
    if !Path::new(input).exists() {
        return Err(format!("Input file does not exist: {input}").into());
    }

    // Create video directory
    let video_dir = format!("./{video_id}");
    fs::create_dir_all(&video_dir)?;

    let output_m3u8 = format!("{video_dir}/{video_id}.m3u8",);
    let segment_pattern = format!("{video_dir}/{video_id}%d.ts");

    println!("Starting conversion: {input} -> {output_m3u8}");
    println!("Video ID: {video_id}");

    let output = Command::new("ffmpeg")
        .args([
            "-hwaccel",
            "auto",
            "-i",
            input,
            "-hls_time",
            "10",
            "-hls_list_size",
            "0",
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-f",
            "hls",
            "-hls_segment_filename",
            &segment_pattern,
            &output_m3u8,
        ])
        .output()?;

    if output.status.success() {
        println!("Conversion successful!");
        println!("Playlist: {output_m3u8}");
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg conversion failed:\n{error_msg}").into());
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let video_id = generate_video_id();

    match convert(&args.input, &video_id) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
