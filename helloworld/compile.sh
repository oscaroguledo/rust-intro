#!/bin/bash

# Find all .rs files in the current directory
rust_files=$(ls *.rs)

# Check if there are no .rs files
if [ -z "$rust_files" ]; then
    echo "No Rust files found!"
    exit 1
fi

# If there is exactly one .rs file, use its name by default or allow the user to specify the name
if [ $(echo "$rust_files" | wc -w) -eq 1 ]; then
    # Extract the filename without the .rs extension
    default_name=$(basename "$rust_files" .rs)
    echo "Found single Rust file: $rust_files"
    echo "Specify an executable name (default: $default_name):"
    read appname

    # If the user doesn't specify a name, use the default (name of the file)
    if [ -z "$appname" ]; then
        appname=$default_name
    fi

    # Use the default or specified file for compilation
    compile_file="$rust_files"
else
    # If more than one file, ask for the appname and file to compile
    echo "Multiple Rust files found. Please specify the file to compile from the following options:"
    echo "$rust_files"
    read selected_file

    # Ensure the selected file is valid
    if [[ ! " $rust_files " =~ " $selected_file " ]]; then
        echo "Invalid file selected!"
        exit 1
    fi

    # Ask for an executable name
    echo "Please specify an appname (default: $selected_file):"
    read appname

    # If the user doesn't specify a name, use the default (name of the selected file)
    if [ -z "$appname" ]; then
        appname=$(basename "$selected_file" .rs)
    fi

    # Use the selected file for compilation
    compile_file="$selected_file"
fi

# Compile the selected file with the specified appname
rustc -o "$appname" "$compile_file"

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Compilation successful. Run './$appname' to start."
    # Run the compiled executable
    ./$appname
else
    echo "Compilation failed."
    exit 1
fi
