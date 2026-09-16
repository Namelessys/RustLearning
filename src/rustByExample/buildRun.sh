#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <file.rs>"
    exit 1
fi

file="$1"
output="${file%.rs}"

rustc "$file" -o "$output"

if [ $? -eq 0 ]; then
    ./"$output"
fi
