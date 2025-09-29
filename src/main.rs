use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::{Duration, Instant};

mod image_ops;
mod model;
mod random_sort;

use model::Model;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SDL2 - Pi5 optimized
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window optimized for Pi5 HDMI display
    let window = video_subsystem
        .window("Pixelsort - Pi5 SDL2", WIDTH, HEIGHT)
        .position_centered()
        .build()?;

    // Create canvas with hardware acceleration on Pi5
    let mut canvas = window
        .into_canvas()
        .accelerated() // Use Pi5 VideoCore VII GPU
        .present_vsync() // Sync with display refresh
        .build()?;

    // Create texture for efficient pixel rendering on Pi5
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, WIDTH, HEIGHT)?;

    let mut model = Model::new(WIDTH, HEIGHT);
    let mut event_pump = sdl_context.event_pump()?;

    // Pi5 optimization: 30 FPS target for smooth performance
    let target_fps = 30;
    let frame_duration = Duration::from_millis(1000 / target_fps);
    let mut last_update = Instant::now();

    'running: loop {
        let frame_start = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    match keycode {
                        Keycode::Up => {
                            model.increase_brightness();
                        }
                        Keycode::Down => {
                            model.decrease_brightness();
                        }
                        Keycode::M => {
                            model.switch_sort_mode();
                        }
                        Keycode::N => {
                            model.switch_direction();
                        }
                        Keycode::B => {
                            model.toggle_random_exclude();
                        }
                        Keycode::Return => {
                            let filename = model.save_current_iteration();
                            println!("Saved: {}", filename);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Pi5 optimization: Only update if enough time has passed
        let now = Instant::now();
        if now.duration_since(last_update) >= frame_duration {
            model.update();
            last_update = now;

            // Render to texture using Pi5 hardware acceleration
            texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                model.render_sdl2(buffer, pitch);
            })?;

            // Present frame using Pi5 GPU
            canvas.clear();
            canvas.copy(&texture, None, None)?;
            canvas.present();
        }

        // Frame rate limiting
        let frame_time = frame_start.elapsed();
        if frame_time < frame_duration {
            std::thread::sleep(frame_duration - frame_time);
        }
    }

    Ok(())
}
