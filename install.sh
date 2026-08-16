#!/bin/bash
set -e

echo "--- Installing cpm (c00l-P4CK4G3-M4N4G3R) ---"

if [ ! -f "cpm" ]; then
    echo "Error: 'cpm' binary not found in current directory."
    exit 1
fi

chmod +x cpm
sudo cp cpm /usr/local/bin/cpm

echo "Successfully installed! Run 'cpm' in your terminal."
