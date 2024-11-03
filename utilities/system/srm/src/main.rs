use std::{
    fs,
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn collector_path() -> PathBuf {
let mut path = std::env::temp_dir();
path.push("collector");
path
}

fn init_collector() -> io::Result<()> {
    let path = collector_path();
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }
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

        let new_file_name = format!("{}_{}", file_name.to_string_lossy(), timestamp);
        collector_file = collector_path().join(new_file_name);
    }

    fs::rename(file_path, &collector_file)?;
    println!("Moved {} to collector as {}", file_path, collector_file.display());
    Ok(())
}
fn list_collector_info() -> io::Result<()> {
    init_collector()?;
    let mut total_size = 0;
    let entries = fs::read_dir(collector_path())?;

    println!("Files in collector:");
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_size = metadata.len();
        total_size += file_size;
        let file_name = entry.file_name();
        println!(
            "{} - {} bytes",
            file_name.to_string_lossy(),
            file_size
        );
    }
    println!("Total size: {} bytes", total_size);
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

fn restore_file(file_name: &str, original_path: &str) -> io::Result<()> {
    let collector_file = collector_path().join(file_name);
    if collector_file.exists() {
        fs::rename(collector_file, original_path)?;
        println!("Restored {} to {}", file_name, original_path);
    } else {
        eprintln!("File not found in collector.");
    }
    Ok(())
}

fn display_help() {
    println!(
        "srm - Safe Remove Tool\n\
        \n\
        Usage:\n\
        \t srm <file_path>                 Move a file to the collector.\n\
        \t srm --info                      Display information about the collector's contents.\n\
        \t srm --unlink <file_name>        Delete a specific file from the collector.\n\
        \t srm --unlink *                  Delete all files from the collector with confirmation.\n\
        \t srm --restore <file_name> <original_path>  Restore a file to its original path.\n\
        \t srm --help | -h                 Display this help message.\n\
        \n\
        Examples:\n\
        \t srm myfile.txt\n\
        \t srm --info\n\
        \t srm --unlink myfile.txt\n\
        \t srm --unlink *\n\
        \t srm --restore myfile.txt /path/to/restore/\n"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--help") | Some("-h") => display_help(),
        Some("srm") if args.len() > 2 => {
            let file_path = &args[2];
            if let Err(e) = move_to_collector(file_path) {
                eprintln!("Error moving file: {}", e);
            }
        }
        Some("--info") => {
            if let Err(e) = list_collector_info() {
                eprintln!("Error listing collector info: {}", e);
            }
        }
        Some("--unlink") if args.len() > 2 => {
            let file_name = &args[2];
            if file_name == "*" {
                if let Err(e) = unlink_all_from_collector() {
                    eprintln!("Error deleting all files: {}", e);
                }
            } else {
                if let Err(e) = unlink_from_collector(file_name) {
                    eprintln!("Error unlinking file: {}", e);
                }
            }
        }
        Some("--restore") if args.len() > 3 => {
            let file_name = &args[2];
            let original_path = &args[3];
            if let Err(e) = restore_file(file_name, original_path) {
                eprintln!("Error restoring file: {}", e);
            }
        }
        _ => {
            eprintln!("Invalid command or arguments.
                       Use --help or -h for usage information.");
            display_help();
        }
    }
}

