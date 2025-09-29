use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod image_ops;
mod model;
mod random_sort;

use model::Model;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Pixelsort - Pi5 HDMI")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut model = Model::new(WIDTH, HEIGHT);
    
    // Pi5 optimization: Frame rate limiting to prevent CPU overload
    use std::time::{Duration, Instant};
    let target_fps = 30; // 30 FPS for smooth performance on Pi5
    let frame_duration = Duration::from_millis(1000 / target_fps);
    let mut last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                model.increase_brightness();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Down) {
                model.decrease_brightness();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::M) {
                model.switch_sort_mode();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::N) {
                model.switch_direction();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::B) {
                model.toggle_random_exclude();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Return) {
                let filename = model.save_current_iteration();
                println!("Saved: {}", filename);
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Pi5 optimization: Only update/render if enough time has passed
            let now = Instant::now();
            if now.duration_since(last_update) >= frame_duration {
                model.update();
                window.request_redraw();
                last_update = now;
            }
        }

        match event {
            Event::RedrawRequested(_) => {
                // Pi5 optimization: Batch rendering operations
                model.render(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    })
}
