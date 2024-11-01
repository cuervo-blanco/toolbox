#!/bin/bash

FILES=()
DESTINATION=""
OPERATION="move"  # Default operation

while [[ "$#" -gt 0 ]]; do
    case "$1" in
        -h|--help)
            echo "Usage:"
            echo "mf -f file1 file2 ... -d destination [-c]"
            echo "-c or --copy to copy files/directories instead of moving"
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

