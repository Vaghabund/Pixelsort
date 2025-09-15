use std::env;

// Module importieren
mod image_ops;
mod model;
mod random_sort;
mod framebuffer;

// Verwendete Typen und Funktionen importieren
use model::Model;
use framebuffer::FrameBuffer;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 320;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if we're running on Pi with framebuffer
    let use_framebuffer = env::var("USE_FRAMEBUFFER").unwrap_or_default() == "1";
    
    if use_framebuffer {
        return run_framebuffer_mode();
    }
    
    // Fall back to normal window mode
    run_window_mode()
}

fn run_framebuffer_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running in framebuffer mode for TFT display...");
    println!("Controls: Arrow Up/Down = brightness, Enter = save, N = direction, M = mode, B = random, Esc = quit");
    
    let mut model = Model::new();
    let mut framebuffer = FrameBuffer::new("/dev/fb0", WIDTH, HEIGHT)?;
    
    let mut frame_buffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    
    // Try to find keyboard device (only on Unix)
    use std::thread;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    let model_arc = Arc::new(Mutex::new(model));
    let running = Arc::new(Mutex::new(true));

    #[cfg(unix)]
    {
    use evdev::{Device, EventType, InputEventKind, Key};

        // Try to find keyboard device
        let keyboard_device = {
            let mut keyboard = None;
            if let Ok(entries) = std::fs::read_dir("/dev/input") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
                        if fname.starts_with("event") {
                            if let Ok(device) = Device::open(&path) {
                                if device.supported_events().contains(EventType::KEY) {
                                    println!("Found keyboard device: {:?}", path);
                                    keyboard = Some(device);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            keyboard
        };

        // Keyboard handling thread (Unix)
        if let Some(mut device) = keyboard_device {
            let model_clone_for_thread = model_arc.clone();
            let running_clone_for_thread = running.clone();
            thread::spawn(move || {
                loop {
                    if !*running_clone_for_thread.lock().unwrap() {
                        break;
                    }

                    match device.fetch_events() {
                        Ok(events) => {
                            for event in events {
                                if let InputEventKind::Key(key) = event.kind() {
                                    if event.value() == 1 { // Key pressed
                                        let mut model = model_clone_for_thread.lock().unwrap();
                                        match key {
                                            Key::KEY_ESC => {
                                                *running_clone_for_thread.lock().unwrap() = false;
                                                return;
                                            },
                                            Key::KEY_UP => {
                                                model.increase_brightness();
                                                println!("Brightness: {}", if model.vertical_mode { 
                                                    model.brightness_value_vertical 
                                                } else { 
                                                    model.brightness_value 
                                                });
                                            },
                                            Key::KEY_DOWN => {
                                                model.decrease_brightness();
                                                println!("Brightness: {}", if model.vertical_mode { 
                                                    model.brightness_value_vertical 
                                                } else { 
                                                    model.brightness_value 
                                                });
                                            },
                                            Key::KEY_ENTER => {
                                                let filename = model.save_current_iteration();
                                                println!("Saved: {}", filename);
                                            },
                                            Key::KEY_N => {
                                                model.switch_direction();
                                                println!("Direction: {}", if model.vertical_mode { "Vertical" } else { "Horizontal" });
                                            },
                                            Key::KEY_M => {
                                                model.switch_sort_mode();
                                                println!("Sort mode: {:?}", model.sort_mode);
                                            },
                                            Key::KEY_B => {
                                                model.toggle_random_exclude();
                                                println!("Random mode: {}", if model.random_exclude_mode { "ON" } else { "OFF" });
                                            },
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                }
            });
        } else {
            println!("Warning: No keyboard device found (Unix). Use Ctrl+C to exit.");
        }
    }
    
    // Main render loop
    while *running.lock().unwrap() {
        {
            let mut model = model_arc.lock().unwrap();
            model.update();
            model.render(&mut frame_buffer);
        }
        framebuffer.write_frame(&frame_buffer)?;
        
        // Control frame rate
        thread::sleep(Duration::from_millis(16)); // ~60fps
    }
    
    println!("Pixelsort terminated.");
    Ok(())
}

fn run_window_mode() -> Result<(), Box<dyn std::error::Error>> {
    // For now, just run a simple console version when not in framebuffer mode
    println!("Window mode not implemented for this Pi setup. Use USE_FRAMEBUFFER=1");
    
    let mut model = Model::new();
    
    // Simple console interaction
    use std::io::{self, Write};
    loop {
        print!("Commands: (q)uit, (u)p brightness, (d)own brightness, (s)ave, (n)ext direction, (m)ode, (r)andom: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "q" => break,
            "u" => {
                model.increase_brightness();
                println!("Brightness increased");
            },
            "d" => {
                model.decrease_brightness();
                println!("Brightness decreased");
            },
            "s" => {
                model.update();
                let filename = model.save_current_iteration();
                println!("Saved: {}", filename);
            },
            "n" => {
                model.switch_direction();
                println!("Direction switched");
            },
            "m" => {
                model.switch_sort_mode();
                println!("Sort mode switched");
            },
            "r" => {
                model.toggle_random_exclude();
                println!("Random mode toggled");
            },
            _ => println!("Unknown command"),
        }
    }
    
    Ok(())
}
