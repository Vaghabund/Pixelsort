// midi.rs
// MIDI-Handling für Pixel-Sorting-Projekt

use midir::{MidiInput, Ignore};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct MidiState {
    pub threshold: Arc<Mutex<u8>>, // Schwellenwert für aktuellen Modus (0-255)
    pub mode_switch_trigger: Arc<Mutex<bool>>, // Button: Modi durchschalten
    pub direction_switch_trigger: Arc<Mutex<bool>>, // Button: Horizontal/Vertikal wechseln
    pub random_toggle_trigger: Arc<Mutex<bool>>, // Button: Random-Exclude toggle
    pub save_trigger: Arc<Mutex<bool>>, // Button: Speichern
}

impl MidiState {
    pub fn new() -> Self {
        MidiState {
            threshold: Arc::new(Mutex::new(60)), // Standard-Threshold
            mode_switch_trigger: Arc::new(Mutex::new(false)),
            direction_switch_trigger: Arc::new(Mutex::new(false)),
            random_toggle_trigger: Arc::new(Mutex::new(false)),
            save_trigger: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_listening(&self) {
        let threshold = self.threshold.clone();
        let mode_switch_trigger = self.mode_switch_trigger.clone();
        let direction_switch_trigger = self.direction_switch_trigger.clone();
        let random_toggle_trigger = self.random_toggle_trigger.clone();
        let save_trigger = self.save_trigger.clone();
        
        thread::spawn(move || {
            let mut midi_in = MidiInput::new("pixelsort-midi").unwrap();
            midi_in.ignore(Ignore::None);
            let in_ports = midi_in.ports();
            if in_ports.is_empty() {
                println!("Kein MIDI-Eingang gefunden!");
                return;
            }
            
            // Launch Control erkennen oder ersten Port nutzen
            let in_port = &in_ports[0];
            println!("MIDI-Port verbunden: {}", midi_in.port_name(in_port).unwrap_or("Unbekannt".to_string()));
            
            let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_, message, _| {
                if message.len() == 3 {
                    let status = message[0] & 0xF0;
                    let note_or_cc = message[1];
                    let value = message[2];
                    
                    // Control Change Nachrichten (0xB0) - Regler
                    if status == 0xB0 {
                        match note_or_cc {
                            21 => { // Erster Regler - Threshold
                                let mut t = threshold.lock().unwrap(); 
                                *t = (value as f32 * 255.0 / 127.0) as u8; // 0-127 zu 0-255 skalieren
                                println!("Threshold: {}", *t);
                            },
                            _ => {
                                println!("Unbekannter CC: {} mit Wert: {}", note_or_cc, value);
                            }
                        }
                    }
                    // Note On Nachrichten (0x90) - Buttons
                    else if status == 0x90 && value > 0 { // Note On mit Velocity > 0
                        match note_or_cc {
                            9 => { // Button 1 - Mode Switch
                                let mut mode_switch = mode_switch_trigger.lock().unwrap();
                                *mode_switch = true;
                                println!("MODE SWITCH gedrückt!");
                            },
                            10 => { // Button 2 - Direction Switch
                                let mut dir_switch = direction_switch_trigger.lock().unwrap();
                                *dir_switch = true;
                                println!("DIRECTION SWITCH gedrückt!");
                            },
                            11 => { // Button 3 - Random Toggle
                                let mut random_toggle = random_toggle_trigger.lock().unwrap();
                                *random_toggle = true;
                                println!("RANDOM TOGGLE gedrückt!");
                            },
                            12 => { // Button 4 - Save
                                let mut save = save_trigger.lock().unwrap();
                                *save = true;
                                println!("SAVE gedrückt!");
                            },
                            _ => {
                                println!("Unbekannter Button: Note {} (Velocity: {})", note_or_cc, value);
                            }
                        }
                    }
                }
            }, ()).unwrap();
            
            println!("MIDI-Listener gestartet.");
            println!("Layout: Regler1=Threshold, Button1=Mode, Button2=Direction, Button3=Random, Button4=Save");
            loop { 
                std::thread::sleep(std::time::Duration::from_millis(100)); 
            }
        });
    }
}
