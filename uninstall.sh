#!/bin/sh
set -e

echo "Uninstalling Jukebox..."

# Remove binary
BINARY=$(which music 2>/dev/null || true)
if [ -n "$BINARY" ]; then
    echo "Removing binary at $BINARY"
    sudo rm "$BINARY"
else
    echo "Binary not found on PATH, skipping"
fi

# Remove data directory
case "$(uname)" in
    Linux)
        DATA_DIR="$HOME/.local/share/jukebox"
        ;;
    Darwin)
        DATA_DIR="$HOME/Library/Application Support/jukebox"
        ;;
    *)
        echo "Unsupported OS"
        exit 1
        ;;
esac

if [ -d "$DATA_DIR" ]; then
    echo "Removing data at $DATA_DIR"
    rm -rf "$DATA_DIR"
else
    echo "No data directory found, skipping"
fi

echo "Jukebox uninstalled."
