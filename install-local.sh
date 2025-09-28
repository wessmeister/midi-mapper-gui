#!/bin/bash
set -e

# Stop running instances
pkill -f midi-mapper-linux 2>/dev/null || true

# Build
echo "Building..."
cargo build --release --locked

# Install
echo "Installing..."
mkdir -p ~/.local/bin ~/.local/share/icons/hicolor/scalable/apps ~/.local/share/applications
cp -f target/release/midi-mapper-linux ~/.local/bin/
chmod +x ~/.local/bin/midi-mapper-linux
cp -f assets/icons/midi-mapper-linux.svg ~/.local/share/icons/hicolor/scalable/apps/

# Desktop entry
cat > ~/.local/share/applications/midi-mapper-linux.desktop << EOF
[Desktop Entry]
Name=MIDI Mapper
Comment=Map MIDI controller inputs to system actions
Exec=midi-mapper-linux
Icon=midi-mapper-linux
Terminal=false
Type=Application
Categories=Audio;AudioVideo;
Keywords=midi;controller;volume;audio;
EOF

# Clean build artifacts (keep deps and build cache for fast rebuilds)
rm -rf target/debug target/release/incremental 2>/dev/null || true
rm -f target/release/*.d target/release/*.rlib target/release/deps/*.d 2>/dev/null || true

# Update caches
gtk-update-icon-cache ~/.local/share/icons/hicolor 2>/dev/null || true
update-desktop-database ~/.local/share/applications 2>/dev/null || true

echo "Done. Run 'midi-mapper-linux' to start."