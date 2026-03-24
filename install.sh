#!/usr/bin/env bash
set -euo pipefail

APP_NAME="jukebox"
INSTALL_DIR="/usr/local/bin"
REPO="dposideon/jukebox"

# ── Detect OS and arch ──────────────────────────────────────────────
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
  x86_64)        ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *)
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

case "$OS" in
  linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  *)
    echo "❌ Unsupported OS: $OS"
    exit 1
    ;;
esac

DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/jukebox-${TARGET}"

echo "🎵 Jukebox Installer"
echo ""
echo "  Downloading for $TARGET..."
curl -fsSL "$DOWNLOAD_URL" -o "/tmp/$APP_NAME"
chmod +x "/tmp/$APP_NAME"

echo "  Installing to $INSTALL_DIR (may require your password):"
sudo mkdir -p "$INSTALL_DIR"
sudo mv "/tmp/$APP_NAME" "$INSTALL_DIR/$APP_NAME"

# ── Set cap on Linux so port 80 works without root ──────────────────
if [ "$OS" = "linux" ]; then
  sudo setcap 'cap_net_bind_service=+ep' "$INSTALL_DIR/$APP_NAME"
fi

# ── Create launcher script ──────────────────────────────────────────
sudo tee "$INSTALL_DIR/music" >/dev/null <<'EOF'
#!/usr/bin/env bash
if [ "$(uname)" = "Darwin" ]; then
    exec sudo JUKEBOX_HOME="$HOME" /usr/local/bin/jukebox "$@"
else
    exec /usr/local/bin/jukebox "$@"
fi
EOF
sudo chmod +x "$INSTALL_DIR/music"

# ── Create app data directories ─────────────────────────────────────
if [ "$OS" = "darwin" ]; then
  APP_DATA_DIR="$HOME/Library/Application Support/jukebox"
else
  APP_DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/jukebox"
fi

mkdir -p "$APP_DATA_DIR/libs"
mkdir -p "$APP_DATA_DIR/output"

# ── Done ────────────────────────────────────────────────────────────
echo ""
echo "  ✅ Jukebox installed!"
echo ""
echo "  📁 Data directory: $APP_DATA_DIR"
echo ""
echo "  Run it anytime with:"
echo ""
echo "      music"
echo ""
echo "  Update later with:"
echo ""
echo "      music update"
echo ""
echo "  Then open http://music.local on any device on your WiFi."
echo ""
