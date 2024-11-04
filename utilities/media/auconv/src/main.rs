use clap::Parser;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    source_file: String,
    format: String,
}

fn main() {
    let args = Args::parse();
    let input_path = &args.source_file;
    let output_format = &args.format;

    let file_stem = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let output_path = format!("{}.{}", file_stem, output_format);

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input_path)
        .arg(&output_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute ffmpeg command");

    if !status.success() {
        eprintln!("ffmpeg command failed with status: {}", status);
        std::process::exit(1);
    }
    println!("Conversion successful. Output file: {}", output_path);
}

