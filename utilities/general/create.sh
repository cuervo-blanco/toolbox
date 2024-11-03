#!/bin/bash

if [ "$#" -eq 0 ]; then
    echo "Usage: create <filename1> [<filename2> ...]"
    exit 1
fi
for file in "$@"; do
    if [ -e "$file" ]; then
        echo "Error: File '$file' already exists."
        exit 1
    else
        > "$file"
    fi
done

