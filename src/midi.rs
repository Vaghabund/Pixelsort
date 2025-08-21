// midi.rs
// MIDI-Handling für Pixel-Sorting-Projekt

use midir::{MidiInput, Ignore};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct MidiState {
    pub brightness: Arc<Mutex<u8>>,
    pub mode: Arc<Mutex<u8>>, // 0 = Brightness, 1 = Black, 2 = White
    pub direction: Arc<Mutex<u8>>, // 0 = Horizontal, 1 = Vertical
}

impl MidiState {
    pub fn new() -> Self {
        MidiState {
            brightness: Arc::new(Mutex::new(60)),
            mode: Arc::new(Mutex::new(0)),
            direction: Arc::new(Mutex::new(0)),
        }
    }

    pub fn start_listening(&self) {
        let brightness = self.brightness.clone();
        let mode = self.mode.clone();
        let direction = self.direction.clone();
        thread::spawn(move || {
            let mut midi_in = MidiInput::new("pixelsort-midi").unwrap();
            midi_in.ignore(Ignore::None);
            let in_ports = midi_in.ports();
            if in_ports.is_empty() {
                println!("Kein MIDI-Eingang gefunden!");
                return;
            }
            let in_port = &in_ports[0];
            let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_, message, _| {
                if message.len() == 3 {
                    let status = message[0] & 0xF0;
                    let cc = message[1];
                    let value = message[2];
                    if status == 0xB0 {
                        match cc {
                            1 => { let mut b = brightness.lock().unwrap(); *b = value; }, // CC1: Brightness
                            2 => { let mut m = mode.lock().unwrap(); *m = value % 3; }, // CC2: Mode
                            3 => { let mut d = direction.lock().unwrap(); *d = value % 2; }, // CC3: Direction
                            _ => {}
                        }
                    }
                }
            }, ()).unwrap();
            loop { std::thread::sleep(std::time::Duration::from_millis(100)); }
        });
    }
}
