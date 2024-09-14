#!/bin/bash

# Directory to search for .html files (current directory by default)
DIRECTORY="${1:-.}"

# Find all .html files and open them in the default web browser
find "$DIRECTORY" -type f -name "*.html" -exec open {} \;
