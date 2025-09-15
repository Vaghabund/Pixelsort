use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// Module importieren
mod image_ops;
mod model;
mod random_sort;

// Verwendete Typen und Funktionen importieren
use model::Model;

const WIDTH: u32 = 480;
const HEIGHT: u32 = 320;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Pixelsort")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut model = Model::new();
    
    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                // Create pixels instance for this frame
                let window_size = window.inner_size();
                let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
                let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
                
                model.update();
                model.render(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render() failed: {}", err);
                    elwt.exit();
                    return;
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }

        if input.update(&event) {
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            if input.key_pressed(KeyCode::ArrowUp) {
                model.increase_brightness();
            }
            if input.key_pressed(KeyCode::ArrowDown) {
                model.decrease_brightness();
            }
            if input.key_pressed(KeyCode::Enter) {
                let saved_file = model.save_current_iteration();
                println!("Gespeichert: {}", saved_file);
            }
            if input.key_pressed(KeyCode::KeyN) {
                model.switch_direction();
            }
            if input.key_pressed(KeyCode::KeyM) {
                model.switch_sort_mode();
            }
            if input.key_pressed(KeyCode::KeyB) {
                model.toggle_random_exclude();
            }
        }
    });

    Ok(())
}
