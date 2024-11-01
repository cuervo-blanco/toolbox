## Toolbox 系統工具箱。

My own personal utitlies toolbox for file system management, system checks and
automation.

This is still in work mode, this is a repository for keeping my tools.
Feel free to chime in!

## Index

### Scripts

#### mf
    Usage:
    mf -f file -d destination -[c]

    A parallel version of mv that simply moves any number of inodes to a single
    destination directory.

    -f --file
      You can pile your inodes.
      mf --file [i-node] [(user-readable, i-node)] .. N - 1
      where N is the total number of piled files
      mf -f [file] ./directory-1 ./directory-2/[file] .. N-1 -d [destination-filepath]

    -d --directory
      Determines the destination filepath
      mf -f [file] .. N-1  --directory ./my-directory

    -c --copy [optional]
      Copy the files
      mf -f [file] .. N-1 -d [destination] --copy

    Still in trial-error mode, use at your discretion.
