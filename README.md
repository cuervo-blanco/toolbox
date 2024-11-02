## Toolbox 系統工具箱。

My own personal utitlies toolbox for file system management, system checks and
automation.

This is still in work mode, this is a repository for keeping my tools.
Feel free to chime in!

## Index
    Utilities:
        General:
            $: mf - Parallel file mover
            $: mdconverter - Markdown to document converter
            $: rename - Batch rename files
        Security:
        Media:
            $: mediaconv - Media Converter
            $:


### Utilities

#### mf [move-files]
```plaintext

USAGE:
    mf [-h | --help] -f FILE -d DESTINATION [-c | --copy]

DESCRIPTION:
    A parallel version of mv to move any number of inodes (files & directories)
    to a single destination directory.

OPTIONS:
    -h, --help
        Show this help.

    -f, --files
        Specify files and directories to move.
        Example: mf -f [file] [file] .. N -d [destination]

    -d, --directory
        Set the destination directory path.
        Example: mf -f [file] .. N --directory ./my-directory

    -c, --copy (optional)
        Copy the files instead of moving them.
        Example: mf -f [file] .. N -d [destination] --copy

NOTES:
    Still in trial-error mode; use with discretion.
```
#### mdconvert - A Minimal Markdown to Document Conversion Tool

```plaintext
USAGE:
    mdconvert [SOURCE FILE] [FORMAT]

DESCRIPTION:
    Converts a Markdown (.md) file to a specified document format.
    Supported formats include:
        - docx (Microsoft Word)
        - pdf (Portable Document Format)
        - html (HyperText Markup Language)
        - odt (OpenDocument Text)
        - rtf (Rich Text Format)
        - txt (Plain Text)
        - epub (Electronic Publication)

ARGUMENTS:
    SOURCE FILE    The path to the Markdown (.md) file to be converted.
    FORMAT         The target format for the output file (e.g., "docx").

EXAMPLES:
    # Convert a file from .md to .docx
    mdconvert readme.md docx

    # Convert a file from .md to .pdf
    mdconvert guide.md pdf

OUTPUT:
    On success, creates a new file with the specified format in the same
    directory as the source file. The output file name is generated from
    the source file name with the target format as the new extension.

NOTES:
    - Ensure Pandoc is installed and accessible in your PATH.
    - Unsupported formats will return an error message.

EXIT CODES:
    0   Success
    1   Error (e.g., unsupported format or missing arguments)
```

### UNIMPLEMENTED

#### rename - Batch rename multiple files with flexible options
```plaintext

USAGE:
    rename [CODE] [OPTIONS] [FILES]...

DESCRIPTION:
    Rename multiple files in batch with options for appending,
    prepending, adding counters, changing extensions, and more.
    Supports an optional CODE to control specific rename behaviors.

OPTIONS:
    CODE         Optional 3-digit control code:
                   - 1st digit: forced rename (0 = off, 1 = on)
                   - 2nd digit: append string (0 = off, 1 = on)
                   - 3rd digit: add counter (0 = off, 1 = on)

                   Example: CODE 101 forces renaming and adds a counter.

    -a, --append STRING      Append STRING to filenames.
    -p, --prepend STRING     Prepend STRING to filenames.
    -c, --counter [POS] [FMT] Add a counter (POS as prefix/suffix, FMT as digits).
    -d, --digits DIGITS      Set counter digits (default is 2).
    -u, --separator SEP      Set separator between counter and name (default: -).
    -r, --replace EXT        Change file extensions to EXT.
    -n, --new-name NAME      Rename all files to NAME with unique counters.
    -f, --follow             Counter follows format.
    -h, --help               Show this help.
    --version                Show version information.

EXAMPLES:
    rename 000 file1.txt file2.txt       # No changes
    rename 100 -n newname.txt oldname.txt # Force rename
    rename 110 -a _v doc1.txt doc2.txt    # Force rename + append
    rename -a _backup file1.txt           # Append "_backup" to files
    rename -r .jpg *.png                  # Replace .png with .jpg

SEE ALSO:
    mv, cp, ls

NOTES:
    CODE: Flags override CODE if both are set.
```
