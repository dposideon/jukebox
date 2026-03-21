#!/usr/bin/env bash
set -euo pipefail

APP_NAME="jukebox"
INSTALL_DIR="/usr/local/bin"

# --- Detect OS and arch ---
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64)  ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

case "$OS" in
    linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

DOWNLOAD_URL="https://github.com/dposideon/jukebox/releases/latest/download/jukebox-${TARGET}"

echo "🎵 Installing Jukebox..."
echo ""

# --- Download binary ---
echo "  Downloading..."
curl -fsSL "$DOWNLOAD_URL" -o "/tmp/$APP_NAME"
chmod +x "/tmp/$APP_NAME"

# --- Install to system path (needs sudo) ---
echo ""
echo "  Installing to $INSTALL_DIR (requires your password):"
sudo mv "/tmp/$APP_NAME" "$INSTALL_DIR/$APP_NAME"

# --- Set cap on Linux so port 80 works without root at runtime ---
if [ "$OS" = "linux" ]; then
    sudo setcap 'cap_net_bind_service=+ep' "$INSTALL_DIR/$APP_NAME"
fi

# --- Create launcher script ---
sudo tee "$INSTALL_DIR/music" > /dev/null << 'EOF'
#!/usr/bin/env bash
if [ "$(uname)" = "Darwin" ]; then
    exec sudo /usr/local/bin/jukebox "$@"
else
    exec /usr/local/bin/jukebox "$@"
fi
EOF
sudo chmod +x "$INSTALL_DIR/music"

echo ""
echo "  ✅ Installed!"
echo ""
echo "  Run it anytime with:"
echo ""
echo "      music"
echo ""
echo "  Then open http://music.local on any device on your WiFi."
echo ""
