# Personal Pixel Sort

Ein modulares Rust-Projekt für kreatives Pixel-Sorting mit der Nannou-Framework.

## Features

- **Mehrere Sortier-Modi**: Brightness, Black, White
- **Horizontal & Vertikal**: Sortierung in beide Richtungen  
- **Live-Brightness-Control**: Echtzeit-Anpassung mit Pfeiltasten
- **Automatic Save**: Speichere Iterationen mit Enter
- **Smart Mode-Switching**: Wechsle Modi ohne Effekt-Überlagerung

## Steuerung

| Taste | Funktion |
|-------|----------|
| `↑`   | Brightness erhöhen |
| `↓`   | Brightness verringern |
| `M`   | Sortier-Modus wechseln (Brightness → Black → White) |
| `N`   | Richtung wechseln (Horizontal ↔ Vertikal) |
| `Enter` | Aktuelle Iteration speichern |
| `Delete` | Letzten Modus wiederherstellen |

## Projekt-Struktur

```
src/
├── main.rs       # Event-Handling & App-Koordination
├── model.rs      # State-Management & Core-Logic  
├── image_ops.rs  # Bildverarbeitung & Sortier-Algorithmen
├── ui.rs         # Display & User-Interface
└── midi.rs       # MIDI-Integration (geplant)
```

## Installation & Start

### Windows/macOS/Linux

```bash
# Repository klonen
git clone https://github.com/Vaghabund/Pixelsort.git
cd personalpixelsort

# Projekt starten
cargo run
```

### Raspberry Pi (automatische Installation)

```bash
# Repository klonen
git clone https://github.com/Vaghabund/Pixelsort.git
cd personalpixelsort

# Automatische Installation ausführen
chmod +x install.sh
./install.sh

# Projekt starten
cargo run --release
```

Das `install.sh`-Skript installiert automatisch:
- Alle benötigten System-Abhängigkeiten
- Rust (falls nicht vorhanden)
- Baut das Projekt im Release-Modus

## Voraussetzungen

- Rust 1.70+ (wird auf Pi automatisch installiert)
- Ein Bild im `assets/`-Ordner

## Verwendung

1. Lege dein Bild als `input.jpg` in den Projekt-Ordner
2. Starte mit `cargo run`
3. Experimentiere mit den verschiedenen Modi und Einstellungen
4. Speichere interessante Iterationen mit `Enter`
5. Ergebnisse findest du im `output/` Ordner

## Dependencies

- `nannou` - Creative Coding Framework
- `image` - Bildverarbeitung
- `midir` - MIDI Support (optional)

## Contributing

Pull Requests und Issues sind willkommen!

## Entwicklung

Dieses Projekt wurde in enger Zusammenarbeit mit GitHub Copilot entwickelt. Die modulare Architektur, Code-Organisation und Implementierung entstanden durch kollaborative Programmierung zwischen Entwickler und KI-Assistent.
