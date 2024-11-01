#!/bin/bash

FILES=()
DESTINATION=""
DIRECTORY=false
NUMBER_OF_ARGUMENTS=0
ARGUMENTS=("$@")  # Store all arguments in an array

for arg in "$@"; do
    ((NUMBER_OF_ARGUMENTS++))
done

while [[ "$#" -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "Usage:"
            echo "mf -f file -d destination"
            echo "use -r if file directory"
            exit 0
            ;;
        -f|--file)
            shift
            while [[ "$1" && "$1" != -* ]]; do
                FILES+=("$1")  # Collect files following -f
                shift
            done
            continue
            ;;
        -d|--destination)
            DESTINATION="$2"
            shift
            ;;
        -r)
            DIRECTORY=true
            ;;
        -rf | -fr )
            DIRECTORY=true
            shift
            while [[ "$1" && "$1" != -* ]]; do
                FILES+=("$1")  # Collect files following -f
                shift
            done
            continue
            ;;
        --)  # End of all options
            shift
            break
            ;;
        -*|--*)  # Catch-all for unrecognized options
            echo "Sorry, option $1 is not recognized."
            echo "Usage: mf -f file -d destination"
            ;;
        *)  # Any remaining arguments (not expected in this context)
            echo "Sorry, argument $1 is not recognized."
            echo "Usage: mf -f file -d destination"
            ;;
    esac
    shift  # Move to the next argument
done

# Check if destination and files are specified
if [[ -z "$DESTINATION" || ${#FILES[@]} -eq 0 ]]; then
    echo "Error: Destination and at least one file must be specified."
    echo "Usage: mf -f file -d destination"
    exit 1
fi

# Move files to destination
if [[ "$DIRECTORY" == true ]]; then
    for file in "${FILES[@]}"; do
        mv "$file" "$DESTINATION"
    done
else
    for file in "${FILES[@]}"; do
        mv "$file" "$DESTINATION"
    done
fi

