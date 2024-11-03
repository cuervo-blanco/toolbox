use std::{
    fs::{self, OpenOptions},
    io::{self, Write, BufRead, BufReader, Read},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
    sync::atomic::{AtomicBool, Ordering},
};
use lazy_static::lazy_static;
use chrono::Local;
use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};

lazy_static! {
    static ref VERBOSE: AtomicBool = AtomicBool::new(false);
}

fn collector_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("collector");
    path
}

fn tracking_log_path() -> PathBuf {
    collector_path().join("collector_log.txt")
}

fn init_collector() -> io::Result<()> {
    let path = collector_path();
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
    let log_path = tracking_log_path();
    if !log_path.exists() {
        fs::File::create(log_path)?; // create the log file if it doesn't exist
    }
    Ok(())
}

fn log_original_path(file_name: &str, original_path: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(tracking_log_path())?;
    writeln!(file, "{}|{}", file_name, original_path)?;
    Ok(())
}

fn read_original_path(file_name: &str) -> io::Result<Option<String>> {
    let file = fs::File::open(tracking_log_path())?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 && parts[0] == file_name {
            return Ok(Some(parts[1].to_string()));
        }
    }
    Ok(None)
}

fn remove_from_log(file_name: &str) -> io::Result<()> {
    let file = fs::File::open(tracking_log_path())?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if !line.starts_with(file_name) {
                Some(line)
            } else {
                None
            }
        })
        .collect();
    fs::write(tracking_log_path(), lines.join("\n"))?;
    Ok(())
}

fn move_to_collector(file_path: &str) -> io::Result<()> {
    init_collector()?;
    let file_name = Path::new(file_path)
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput,
            "Invalid file path"))?;
    let mut collector_file = collector_path().join(file_name);

    if collector_file.exists() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let new_file_name = format!("{}_{}",
            file_name.to_string_lossy(), timestamp);
        collector_file = collector_path().join(new_file_name);
    }

    if VERBOSE.load(Ordering::SeqCst) {
        println!("Moving file from {} to {}", file_path, collector_file.display());
    }

    if VERBOSE.load(Ordering::SeqCst) {
        let metadata = fs::metadata(file_path)?;
        let file_size = metadata.len();
        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut src = fs::File::open(file_path)?;
        let mut dst = fs::File::create(&collector_file)?;

        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = src.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            dst.write_all(&buffer[..bytes_read])?;
            pb.inc(bytes_read as u64);
        }
        pb.finish_with_message("Move completed");
        fs::remove_file(file_path)?;
    } else {
        fs::rename(file_path, &collector_file)?;
    }

    log_original_path(&collector_file.file_name()
        .unwrap().to_string_lossy(), file_path)?;
    println!("Moved {} to collector as {}",
        file_path, collector_file.display());
    Ok(())
}

fn list_collector_info() -> io::Result<()> {
    init_collector()?;
    let mut total_size = 0;
    let entries = fs::read_dir(collector_path())?;
    let log_path = tracking_log_path();
    let mut original_paths = std::collections::HashMap::new();

    if log_path.exists() {
        let file = fs::File::open(log_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                original_paths.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    println!("{:<30} {:<15} {:<60}", "File Name", "File Size", "Original Path");
    println!("{:-<105}", "");

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_size = metadata.len();
        total_size += file_size;
        let file_name = entry.file_name().to_string_lossy().to_string();

        let original_path = original_paths
            .get(&file_name)
            .map(|s| s.as_str())
            .unwrap_or("Unknown");

        println!(
            "{:<30} {:<15} {:<60}",
            file_name,
            format!("{} bytes", file_size),
            original_path
        );
    }

    println!("{:-<105}", "");
    println!("{:<30} {:<15}", "Total size:", format!("{} bytes", total_size));
    Ok(())
}

fn unlink_from_collector(file_name: &str) -> io::Result<()> {
    let file_path = collector_path().join(file_name);

    println!("Are you sure you want to delete {file_name} from collector? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().eq_ignore_ascii_case("y") {
        if file_path.exists() {
            fs::remove_file(&file_path)?;
            println!("Unlinked {}", file_name);
        } else {
            eprintln!("File not found in collector");
        }
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

fn unlink_all_from_collector() -> io::Result<()> {
    init_collector()?;
    println!("Are you sure you want to delete all files in collector? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim().eq_ignore_ascii_case("y") {
        let entries = fs::read_dir(collector_path())?;
        for entry in entries {
            let entry = entry?;
            fs::remove_file(entry.path())?;
        }
        println!("All files deleted from collector.");
    } else {
        println!("Operation cancelled.");
    }
    Ok(())
}

fn restore_file(file_name: &str, destination: Option<&String>) -> io::Result<()> {
    let collector_file = collector_path().join(file_name);
    let target_path = if let Some(dest) = destination {
        PathBuf::from(dest).join(file_name)
    } else {
        let original_path = read_original_path(file_name)?
            .map(PathBuf::from)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound,
                "Original path not found"))?;
        PathBuf::from(&original_path)
    };

    if target_path.exists() {
        println!("File {} already exists at destination.", target_path.display());
        println!("Do you want to overwrite, skip, or rename? (o/s/r)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "o" => {
                fs::remove_file(&target_path)?;
                log_restoration(file_name, &target_path, "Restored and Overwritten")?;
                println!("Ovewriting exisiting file.");
            }
            "s" => {
                println!("Skipping restoration of {}.", file_name);
                log_restoration(file_name, &target_path, "Skipped")?;
                return Ok(());
            }
            "r" => {
                println!("Enter new filename:");

                let new_name = Input::new()
                    .with_prompt("Enter new filename")
                    .default(file_name.to_string())
                    .interact_text().unwrap();
                let new_target_path = target_path.with_file_name(new_name);
                fs::rename(&collector_file, &new_target_path)?;
                remove_from_log(file_name)?;
                println!("Restored {} as {}", file_name, new_target_path.display());
                log_restoration(file_name, &new_target_path, "Renamed and Restored")?;
                return Ok(());
            }
            option => {
                println!("Invalid option. Skipping restoration of {}.", file_name);
                let invalid_option = option;
                log_restoration(
                    file_name,
                    &target_path,
                    &format!("Skipped due to unvalid option: {}", invalid_option),
                )?;
                return Ok(());
            }
        }
    }
    fs::rename(&collector_file, &target_path)?;
    remove_from_log(file_name)?;
    println!("Restored {} to {}", file_name, target_path.display());
    log_restoration(file_name, &target_path, "Restored")?;
    Ok(())
}

fn restoration_log_path() -> PathBuf {
    collector_path().join("restoration_log.txt")
}

fn log_restoration(
    file_name: &str,
    target_path: &Path,
    status: &str) -> io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = OpenOptions::new()
        .append(true).create(true).open(restoration_log_path())?;
    writeln!(
        file,
        "{} | {} | {} | {}",
        timestamp,
        file_name,
        target_path.display(),
        status
    )?;
    Ok(())
}

fn restore_all_files() -> io::Result<()> {
    init_collector()?;
    let entries = fs::read_dir(collector_path())?;

    println!("Are you sure you want to restore all files from the collector? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Operation cancelled.");
        return Ok(());
    }

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        let original_path_option = read_original_path(&file_name)?;
        let original_path = if let Some(path) = original_path_option {
            path
        } else {
            eprintln!("Original path for {} not found. Skipping.", file_name);
            continue;
        };

        if let Err(e) = restore_file(&file_name, Some(&original_path)) {
            eprintln!("Error restoring file {}: {}", file_name, e);
        }
    }
    println!("All files have been restored.");
    Ok(())
}

fn display_help() {
    println!(
        "srm - Safe Remove Tool\n\
        \n\
        Usage:\n\
        \t srm [--verbose] <file_path>                   Move a file to the collector.\n\
        \t srm --list                                    Display information about the collector's contents.\n\
        \t srm --unlink [--verbose] <file_name> [...]    Delete specific files from the collector.\n\
        \t srm --unlink-all                              Delete all files from the collector with confirmation.\n\
        \t srm --restore [--verbose] <file_name> [--destination <path>]  Restore a file to its original or specified path.\n\
        \t srm --restore-all [--verbose]                 Restore all files from the collector to their original paths.\n\
        \t srm --help | -h                               Display this help message.\n\
        \n\
        Options:\n\
        \t --verbose                                     Enable verbose output.\n\
        \n\
        Examples:\n\
        \t srm myfile.txt\n\
        \t srm --list\n\
        \t srm --unlink myfile.txt anotherfile.txt\n\
        \t srm --unlink-all\n\
        \t srm --restore myfile.txt\n\
        \t srm --restore myfile.txt --destination /path/to/restore/\n\
        \t srm --restore-all\n\
        \t srm --verbose myfile.txt\n"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--verbose".to_string()) {
        VERBOSE.store(true, Ordering::SeqCst);
    }

    let args: Vec<String> = args
        .into_iter()
        .filter(|arg| arg != "--verbose")
        .collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--help") | Some("-h") => display_help(),
        Some("--list") => {
            if let Err(e) = list_collector_info() {
                eprintln!("Error listing collector info: {}", e);
            }
        }
        Some("--unlink-all") => {
            if let Err(e) = unlink_all_from_collector() {
                eprintln!("Error deleting all files: {}", e);
            }
        }
        Some("--unlink") if args.len() > 2 => {
            for file in args.iter().skip(2) {
                if let Err(e) = unlink_from_collector(file) {
                    eprintln!("Error unlinking file {}: {}", file, e);
                }
            }
        }
        Some("--restore-all") => {
            if let Err(e) = restore_all_files() {
                eprintln!("Error restoring all files: {}", e);
            }
        }
        Some("--restore") if args.len() > 2 => {
            let mut files = vec![];
            let mut destination = None;
            let mut i = 2;

            while i < args.len() {
                match args[i].as_str() {
                    "--destination" => {
                        i += 1;
                        if i < args.len() {
                            destination = Some(&args[i]);
                        } else {
                            eprintln!("Error: Missing destination path after --destination.");
                            display_help();
                            return;
                        }
                    }
                    file => files.push(file),
                }
                i += 1;
            }
            for file in files {
                if let Err(e) = restore_file(file, destination) {
                    eprintln!("Error restoring file {}: {}", file, e);
                }
            }
        }
        Some(_file_path) => {
            for file in args.iter().skip(1) {
                if let Err(e) = move_to_collector(file) {
                    eprintln!("Error moving file {}: {}", file, e);
                }
            }
        }_ => {
            eprintln!("Invalid command or arguments.
                       Use --help or -h for usage information.");
            display_help();
        }
    }
}

