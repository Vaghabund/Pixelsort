use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::fs;

mod image_ops;
mod model;
mod random_sort;

use model::{Model, SortMode};

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

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            model.render(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                eprintln!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

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

            model.update();
            window.request_redraw();
        }
    });
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img_path = fs::read_dir(&assets)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            if let Some(ext) = path.extension() {
                matches!(
                    ext.to_str().unwrap_or("").to_lowercase().as_str(),
                    "png" | "jpg" | "jpeg" | "bmp" | "gif"
                )
            } else {
                false
            }
        })
        .expect("Kein Bild im assets-Ordner gefunden!");
    let img = image::open(&img_path).expect("Bild konnte nicht geladen werden").to_rgba8();
    let (orig_width, orig_height) = img.dimensions();
    
    // Scale to fit 7-inch screen (assume 1024x600 or similar)
    let max_width = 800;
    let max_height = 480;
    let scale = ((max_width as f32 / orig_width as f32).min(max_height as f32 / orig_height as f32)).min(1.0);
    let width = (orig_width as f32 * scale) as u32;
    let height = (orig_height as f32 * scale) as u32;
    
    let img = image::imageops::resize(&img, width, height, image::imageops::FilterType::Lanczos3);

    app.new_window()
        .size(width, height)
        .title("Pixelsort - HDMI")
        .view(view)
        .build()
        .unwrap();

    let img_horizontal = img.clone();
    let img_vertical = img.clone();
    let texture = wgpu::Texture::from_image(app, &DynamicImage::ImageRgba8(img_horizontal.clone()));

    Model {
        texture,
        img_original: img.clone(),
        img_horizontal,
        img_vertical,
        brightness_value: 60,
        brightness_value_vertical: 60,
        width,
        height,
        needs_resort: true,
        vertical_mode: false,
        last_vertical_mode: false,
        sort_mode: SortMode::Brightness,
        random_exclude_mode: false,
        image_counter: 1,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.needs_resort {
        set_sort_mode(model.sort_mode);
        if !model.vertical_mode {
            sort_and_update_texture(app, model);
        } else {
            vertical_sort_and_update_texture(app, model);
        }
        model.needs_resort = false;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.texture(&model.texture);
    ui::draw_info(&draw, model);
    draw.to_frame(app, &frame).unwrap();
}

fn event(_app: &App, model: &mut Model, event: nannou::Event) {
    if let nannou::Event::WindowEvent { simple, .. } = event {
        if let Some(window_event) = simple {
            match window_event {
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Up) => {
                    model.increase_brightness();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Down) => {
                    model.decrease_brightness();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::Return) => {
                    let saved_file = model.save_current_iteration();
                    println!("Gespeichert: {}", saved_file);
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::N) => {
                    model.switch_direction();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::M) => {
                    model.switch_sort_mode();
                }
                nannou::event::WindowEvent::KeyPressed(nannou::prelude::Key::B) => {
                    model.toggle_random_exclude();
                }
                _ => {}
            }
        }
    }
}
