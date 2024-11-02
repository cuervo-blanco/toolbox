#!/bin/bash

FILES=()
DESTINATION=""
OPERATION="move"  # Default operation

while [[ "$#" -gt 0 ]]; do
    case "$1" in
        -h|--help)
            echo ""
            echo "Quick help:"
            echo "mf -f [file] .. -d [filepath] [-c]"
            echo "-c or --copy to copy files/directories instead of moving"
            echo
            echo "Usage:
            mf [-h --help] -f file -d destination -[c]

            A parallel version of the almighty -- mv -- that simply moves any number
            of inodes (files & directories) to a single destination directory.

            -h --help
                Show this help.

            -f --files
              You can pile your inodes.
              mf --file [i-node] [(user-readable, i-node)] .. N - 1
              where N is the total number of piled files
              mf -f [file] ./directory-1 ./directory-2/[file] .. N-1 -d [filepath]

            -d --directory
              Determines the destination filepath
              mf -f [file] .. N-1  --directory ./my-directory

            -c --copy [optional]
              Copy the files
              mf -f [file] .. N-1 -d [destination] --copy

            Still in trial-error mode, use at your discretion."
            echo ""
            exit 0
            ;;
        -f|--file)
            shift
            while [[ "$1" && "$1" != -* ]]; do
                FILES+=("$1")
                shift
            done
            ;;
        -d|--destination)
            shift
            if [[ "$1" && "$1" != -* ]]; then
                DESTINATION="$1"
                shift
            else
                echo "Error: Destination directory is missing."
                exit 1
            fi
            ;;
        -c|--copy)
            OPERATION="copy"
            shift
            ;;
        --)  # End of all options
            shift
            break
            ;;
        -*|--*)  # Catch-all for unrecognized options
            echo "Sorry, option $1 is not recognized."
            echo "Usage: mf -f file -d destination"
            exit 1
            ;;
        *)  # Any remaining arguments (not expected in this context)
            echo "Sorry, argument $1 is not recognized."
            echo "Usage: mf -f file -d destination"
            exit 1
            ;;
    esac
done

# Check if destination and files are specified
if [[ -z "$DESTINATION" || ${#FILES[@]} -eq 0 ]]; then
    echo "Error: Destination and at least one file must be specified."
    echo "Usage: mf -f file -d destination"
    echo "mf -h or --help for help."
    exit 1
fi

# Perform the operation
for file in "${FILES[@]}"; do
    if [[ "$OPERATION" == "copy" ]]; then
        cp -r "$file" "$DESTINATION"
    else
        mv "$file" "$DESTINATION"
    fi
done
exit 0

