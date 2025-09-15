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
    
    let mut model = Model::new();
    let mut framebuffer = FrameBuffer::new("/dev/fb1", WIDTH, HEIGHT)?; // fb1 for TFT display
    
    // Simple render loop for framebuffer
    let mut frame_buffer = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
    
    loop {
        model.update();
        model.render(&mut frame_buffer);
        framebuffer.write_frame(&frame_buffer)?;
        
        // Simple delay to control frame rate
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
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
