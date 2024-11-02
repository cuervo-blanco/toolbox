use std::env;
use std::path::PathBuf;
use std::process::exit;
use pandoc::{Pandoc, OutputKind, OutputFormat, InputKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: mdconvert [SOURCE FILE] [FORMAT]");
        exit(1);
    }
    let input_file = &args[1];
    let target_format = &args[2];


    let output_file = format!("{}.{}", input_file.trim_end_matches(".md"), target_format);
    let output_file = std::path::PathBuf::from(output_file);

    let output_format = match target_format.as_str() {
        "docx" => OutputFormat::Docx,
        "pdf" => OutputFormat::Pdf,
        "html" => OutputFormat::Html,
        "odt" => OutputFormat::Odt,
        "rtf" => OutputFormat::Rtf,
        "txt" => OutputFormat::Plain,
        "epub" => OutputFormat::Epub,
        other => {
            eprintln!("Unsupported format: {}", other);
            exit(1);
        }
    };

    let mut pandoc = Pandoc::new();
    pandoc.set_input(InputKind::Files(vec![PathBuf::from(input_file)]));
    pandoc.set_output(OutputKind::File(output_file.clone()));
    pandoc.set_output_format(output_format, Vec::new());

    // Run the conversion
    match pandoc.execute() {
        Ok(_) => {
            println!("Conversion successful! Output: {}", output_file.display());
        }
        Err(e) => {
            eprintln!("Conversion failed: {}", e);
            exit(1);
        }
    }
}

