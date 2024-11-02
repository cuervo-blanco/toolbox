```plaintext
RENAME(1)                      User Manual                      RENAME(1)

NAME
       rename - batch rename multiple files with flexible options,
               using an optional control code

SYNOPSIS
       rename [CODE] [OPTIONS] [FILES]...

DESCRIPTION
       The rename utility provides a flexible way to rename multiple
       files in batch. It supports appending or prepending strings,
       adding counters, changing file extensions, and more. It can
       operate with an optional CODE to control specific behaviors,
       providing efficient file management.

OPTIONS

       CODE    An optional three-digit CODE to control rename settings:
                 - First digit: forced renaming (0 = off, 1 = on)
                 - Second digit: append string (0 = off, 1 = on)
                 - Third digit: add counter (0 = off, 1 = on)

                 Example: CODE 101 forces renaming and adds a counter.

       -a, --append STRING
                 Append STRING to filenames.
                 Example: rename -a _backup file1.txt file2.txt

       -p, --prepend STRING
                 Prepend STRING to filenames.
                 Example: rename -p new_ file1.txt file2.txt

       -c, --counter [POSITION] [FORMAT]
                 Add a counter to filenames. Choose POSITION as
                 prefix or suffix, and FORMAT (e.g., -03).
                 Example: rename -c suffix -03 file1.txt file2.txt

       -d, --digits DIGITS
                 Set number of digits for the counter (default is 2).
                 Example: rename -c prefix -d 4 file1.txt file2.txt

       -u, --separator SEPARATOR
                 Set separator between counter and filename (default: -).
                 Example: rename -c suffix -u "-" file1.txt file2.txt

       -r, --replace EXTENSION
                 Change file extensions to EXTENSION.
                 Example: rename -r .md file1.txt file2.txt

       -n, --new-name NEW_NAME
                 Rename all files to NEW_NAME with unique counters.
                 Example: rename -n newfile.txt file1.txt file2.txt

       -f, --follow
                 Flag to indicate counter follows format.
                 Example: rename -f -n newfile.txt myfile-2.txt movie.mov

       -h, --help
                 Display help message.

       --version
                 Display version information.

EXAMPLES

       CODE: 000 (No changes applied)
             rename 000 file1.txt file2.txt

       CODE: 100 (Force renaming without counter or append)
             rename 100 -n newname.txt oldname.txt

       CODE: 110 (Force renaming and append)
             rename 110 -a _v doc1.txt doc2.txt

       CODE: 111 (Force renaming with append and counter)
             rename 111 -a _backup -c suffix -d 2 fileA.txt fileB.txt

       Append a string
             rename -a _backup document.txt report.doc

       Prepend a counter
             rename -c prefix -d 3 file1.txt file2.txt

       Replace extensions
             rename -r .jpg *.png

SEE ALSO
       mv(1), cp(1), ls(1)

NOTES
       CODE: If no CODE is specified, rename behaves according to flags.
       Counters and overwrite options can be managed by CODE or flags,
       with flags taking precedence.

```
