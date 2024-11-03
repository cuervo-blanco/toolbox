
# SRM Installation Guide

This guide explains how to manually install `srm`, a Safe Remove Tool for
managing files securely.

## Prerequisites
- Ensure you have a Rust development environment set up with `cargo`.

## Installation Steps

1. **Clone the Repository**
   ```bash
   git clone https://github.com/cuervo-blanco/srm.git
   cd ./srm
   ```

2. **Build the Project**
   ```bash
   cargo build --release
   ```
   This will create an optimized binary in the `target/release/` directory.

3. **Copy the Binary** (optional)
   If you want to use `srm` from anywhere, copy the binary to a directory
   in your `$PATH`, such as `/usr/local/bin`:

   ```bash
   sudo cp target/release/srm /usr/local/bin/
   ```

4. **Verify Installation**
   ```bash
   srm --help
   ```
   This command should display the `srm` help information,
   confirming the installation was successful.

---

### Optional: Automate Installation with the Provided Script

For convenience, an optional Bash script, `install.sh`, is included in this
repository. Run it to automatically setup Rust, build the binary and copy to
to the appropriate directory (/usr/local/bin for Unix or /usr/bin for Windows)

1. **Run the Installation Script**
   ```bash
   cd srm
   ./install.sh
   ```

2. **Confirm Installation**
   After running the script, you can verify the installation as described above.

---

```
SRM(1)                         Safe Remove Tool                         SRM(1)

NAME
       srm - Safe Remove Tool for moving files to a collector and restoring or
       deleting them

SYNOPSIS
       srm [OPTION]... <file_path>
       srm --list
       srm --show-path
       srm --unlink [OPTION]... <file_name>...
       srm --unlink-all
       srm --restore [OPTION]... <file_name> [--destination <path>]
       srm --restore-all
       srm --help | -h

DESCRIPTION
       The srm command provides a safe way to move files to a "collector" for
       temporary removal, where they can be later restored or permanently
       deleted. Files moved to the collector can be listed, restored, or
       unlinked based on the command provided.

OPTIONS
       --verbose
              Enable verbose output for detailed information on operations.

       --list
              Display information about the collector's contents, including
              file names, sizes, and original paths.

       --show-path
              Display the collector's location in the system

       --unlink <file_name>...
              Delete specified files from the collector.

       --unlink-all
              Delete all files from the collector after confirmation.

       --restore <file_name> [--destination <path>]
              Restore a file to its original or a specified path. If a
              destination is provided, the file will be restored to that path.

       --restore-all
              Restore all files from the collector to their original paths
              with confirmation.

       --help, -h
              Display this help message.

USAGE
       To move a file to the collector:
              srm myfile.txt

       To view collector contents:
              srm --list

       To delete specific files from the collector:
              srm --unlink myfile.txt anotherfile.txt

       To delete all files from the collector with confirmation:
              srm --unlink-all

       To restore a specific file:
              srm --restore myfile.txt

       To restore a specific file to a specified path:
              srm --restore myfile.txt --destination /path/to/restore/

       To restore all files from the collector:
              srm --restore-all

EXAMPLES
       Move a file with verbose output:
              srm --verbose myfile.txt

       List all files in the collector:
              srm --list

       Restore a file to a custom path:
              srm --restore myfile.txt --destination /path/to/restore/

EXIT STATUS
       0      Successful execution.
       1      An error occurred.

FILES
       /tmp/collector/
              Default directory where files are stored temporarily after
              being moved by srm.

       /tmp/collector/collector_log.txt
              Log file for tracking original paths of files moved to the
              collector.

       /tmp/collector/restoration_log.txt
              Log file for recording details of file restorations.

AUTHOR
       Written by cuervo-blanco.

COPYRIGHT
       License under MIT License.

SEE ALSO
       rm(1), mv(1)

                                  SRM Manual
```
