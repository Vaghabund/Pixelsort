#!/bin/bash

echo "Pixelsort Installation für Raspberry Pi"
echo "======================================="

# System-Pakete aktualisieren
echo "Aktualisiere Paketlisten..."
sudo apt update

# Benötigte System-Abhängigkeiten installieren
echo "Installiere System-Abhängigkeiten..."
sudo apt install -y \
    build-essential \
    pkg-config \
    libasound2-dev \
    libgtk-3-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    libgl1-mesa-dev \
    git \
    curl

# Prüfen ob Rust bereits installiert ist
if ! command -v cargo &> /dev/null; then
    echo "Installiere Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust ist bereits installiert"
    source $HOME/.cargo/env
fi

# Rust-Version anzeigen
echo "Rust-Version:"
rustc --version
cargo --version

# Projekt bauen
echo "Baue Pixelsort-Projekt..."
cargo build --release

echo ""
echo "Installation abgeschlossen!"
echo "Starte das Projekt mit: cargo run --release"
echo "Oder verwende die fertige Binary: ./target/release/pixelsort"
echo ""
echo "Stelle sicher, dass ein MIDI-Controller angeschlossen ist"
echo "Lege ein Bild in den 'assets'-Ordner"
