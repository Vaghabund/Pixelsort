use std::env;

// Module importieren
mod image_ops;
mod model;
mod random_sort;
mod framebuffer;

// Verwendete Typen und Funktionen importieren
use model::Model;
use framebuffer::FrameBuffer;

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
    
    // Create model (it will pick up configured width/height from env or default to 480x320)
    let mut model = Model::new();
    let width = model.width;
    let height = model.height;

    // Create framebuffer with the model's dimensions
    let mut framebuffer = FrameBuffer::new("/dev/fb0", width, height, None)?;
    
    // Pixel buffer RGBA8
    let mut frame_buffer = vec![0u8; (width * height * 4) as usize];
    
    // Input handling setup: channel + atomic running flag
    use std::thread;
    use std::sync::{Arc, mpsc, atomic::{AtomicBool, Ordering}};
    use std::time::Duration;

    // Commands produced by input thread and consumed by main loop
    #[derive(Debug, Clone, Copy)]
    enum UiCommand {
        Quit,
        IncreaseBrightness,
        DecreaseBrightness,
        Save,
        SwitchDirection,
        SwitchMode,
        ToggleRandom,
    }

    let (tx, rx) = mpsc::channel::<UiCommand>();
    let running = Arc::new(AtomicBool::new(true));

    #[cfg(unix)]
    {
        use evdev::{Device, EventType, InputEventKind, Key};

        // Try to find keyboard device - prefer actual keyboards over mice
        let keyboard_device = {
            let mut keyboard = None;
            let mut devices = Vec::new();
            
            if let Ok(entries) = std::fs::read_dir("/dev/input") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
                        if fname.starts_with("event") {
                            if let Ok(device) = Device::open(&path) {
                                if device.supported_events().contains(EventType::KEY) {
                                    // Check device name to prefer keyboards over mice
                                    let name = device.name().unwrap_or("").to_lowercase();
                                    let priority = if name.contains("keyboard") { 1 }
                                                 else if name.contains("mouse") { 3 }
                                                 else { 2 };
                                    devices.push((priority, path.clone(), device));
                                }
                            }
                        }
                    }
                }
            }
            
            // Sort by priority (1=keyboard, 2=other, 3=mouse)
            devices.sort_by_key(|(priority, _, _)| *priority);
            
            if let Some((_, path, device)) = devices.into_iter().next() {
                println!("Found keyboard device: {:?}", path);
                keyboard = Some(device);
            }
            
            keyboard
        };

        // Keyboard handling thread (Unix)
        if let Some(mut device) = keyboard_device {
            let tx_clone = tx.clone();
            let running_clone = running.clone();
            thread::spawn(move || {
                loop {
                    if !running_clone.load(Ordering::SeqCst) {
                        break;
                    }

                    match device.fetch_events() {
                        Ok(events) => {
                            for event in events {
                                if let InputEventKind::Key(key) = event.kind() {
                                    if event.value() == 1 { // Key pressed
                                        let cmd = match key {
                                            Key::KEY_ESC => Some(UiCommand::Quit),
                                            Key::KEY_UP => Some(UiCommand::IncreaseBrightness),
                                            Key::KEY_DOWN => Some(UiCommand::DecreaseBrightness),
                                            Key::KEY_ENTER => Some(UiCommand::Save),
                                            Key::KEY_N => Some(UiCommand::SwitchDirection),
                                            Key::KEY_M => Some(UiCommand::SwitchMode),
                                            Key::KEY_B => Some(UiCommand::ToggleRandom),
                                            _ => None,
                                        };
                                        if let Some(c) = cmd {
                                            let _ = tx_clone.send(c);
                                            // Fast-path for quit: also set running flag so thread will exit quickly
                                            if matches!(c, UiCommand::Quit) {
                                                running_clone.store(false, Ordering::SeqCst);
                                                return;
                                            }
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

    // Helper to handle commands in the main thread (mutates model only here)
    let handle_command = |model: &mut Model, cmd: UiCommand, running: &Arc<AtomicBool>| {
        match cmd {
            UiCommand::Quit => {
                running.store(false, Ordering::SeqCst);
            }
            UiCommand::IncreaseBrightness => {
                model.increase_brightness();
                println!("Brightness: {}", if model.vertical_mode { model.brightness_value_vertical } else { model.brightness_value });
            }
            UiCommand::DecreaseBrightness => {
                model.decrease_brightness();
                println!("Brightness: {}", if model.vertical_mode { model.brightness_value_vertical } else { model.brightness_value });
            }
            UiCommand::Save => {
                let filename = model.save_current_iteration();
                println!("Saved: {}", filename);
            }
            UiCommand::SwitchDirection => {
                model.switch_direction();
                println!("Direction: {}", if model.vertical_mode { "Vertical" } else { "Horizontal" });
            }
            UiCommand::SwitchMode => {
                model.switch_sort_mode();
                println!("Sort mode: {:?}", model.sort_mode);
            }
            UiCommand::ToggleRandom => {
                model.toggle_random_exclude();
                println!("Random mode: {}", if model.random_exclude_mode { "ON" } else { "OFF" });
            }
        }
    };

    // Main render loop: own and mutate model here only
    while running.load(Ordering::SeqCst) {
        // Drain input commands
        while let Ok(cmd) = rx.try_recv() {
            handle_command(&mut model, cmd, &running);
        }

        model.update();
        model.render(&mut frame_buffer);

        framebuffer.write_frame(&frame_buffer)?;
        
        // Control frame rate
        thread::sleep(Duration::from_millis(16)); // ~60fps
    }
    
    println!("Pixelsort terminated.");
    Ok(())
}

fn run_window_mode() -> Result<(), Box<dyn std::error::Error>> {
    use pixels::{Pixels, SurfaceTexture};
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        keyboard::KeyCode,
        window::WindowBuilder,
    };
    use winit_input_helper::WinitInputHelper;

    println!("Running in HDMI window mode...");
    println!("Controls: Arrow Up/Down = brightness, M = mode, N = direction, B = random, Enter = save, Esc = quit");
    
    let mut model = Model::new();
    let width = model.width;
    let height = model.height;
    
    // Create window and event loop
    let event_loop = EventLoop::new()?;
    let window = {
        WindowBuilder::new()
            .with_title("Pixelsort - Interactive Pixel Art")
            .with_inner_size(winit::dpi::LogicalSize::new(width * 2, height * 2)) // 2x scale for better visibility
            .build(&event_loop)?
    };

    // Create pixel buffer
    let surface_texture = SurfaceTexture::new(width * 2, height * 2, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;
    let mut input_helper = WinitInputHelper::new();

    // Initial render
    model.update();
    let mut frame_buffer = vec![0u8; (width * height * 4) as usize];
    model.render(&mut frame_buffer);

    let window_id = window.id();
    
    event_loop.run(move |event, elwt| {
        if input_helper.update(&event) {
            // Handle keyboard input
            
            if input_helper.key_pressed(KeyCode::Escape) {
                elwt.exit();
                return;
            }

            let mut needs_update = false;

            if input_helper.key_pressed(KeyCode::ArrowUp) {
                model.increase_brightness();
                needs_update = true;
                println!("Brightness: {}", if model.vertical_mode { model.brightness_value_vertical } else { model.brightness_value });
            }
            if input_helper.key_pressed(KeyCode::ArrowDown) {
                model.decrease_brightness();
                needs_update = true;
                println!("Brightness: {}", if model.vertical_mode { model.brightness_value_vertical } else { model.brightness_value });
            }
            if input_helper.key_pressed(KeyCode::KeyM) {
                model.switch_sort_mode();
                needs_update = true;
                println!("Sort mode: {:?}", model.sort_mode);
            }
            if input_helper.key_pressed(KeyCode::KeyN) {
                model.switch_direction();
                needs_update = true;
                println!("Direction: {}", if model.vertical_mode { "Vertical" } else { "Horizontal" });
            }
            if input_helper.key_pressed(KeyCode::KeyB) {
                model.toggle_random_exclude();
                needs_update = true;
                println!("Random mode: {}", if model.random_exclude_mode { "ON" } else { "OFF" });
            }
            if input_helper.key_pressed(KeyCode::Enter) {
                model.update();
                let filename = model.save_current_iteration();
                println!("Saved: {}", filename);
            }

            if needs_update {
                model.update();
                model.render(&mut frame_buffer);
                
                // Copy to pixel buffer
                let pixel_data = pixels.frame_mut();
                pixel_data.copy_from_slice(&frame_buffer);
                
                // Trigger redraw
                elwt.set_control_flow(ControlFlow::Poll);
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id: id,
            } if id == window_id => {
                if let Err(err) = pixels.render() {
                    eprintln!("Render error: {}", err);
                    elwt.exit();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: id,
            } if id == window_id => {
                elwt.exit();
            }
            _ => {}
        }
    })?;

    Ok(())
}
