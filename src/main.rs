use nannou::{image, prelude::*, LoopMode};
mod rectangle_packer;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1000, 1000)
        .loop_mode(LoopMode::NTimes {
            number_of_updates: 1,
        })
        .run();
}

struct Model {
    tries: u32,
    rectangle_packer: rectangle_packer::RectanglePacker,
    background_image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    background_texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    let boundary = app.window_rect();
    let width = boundary.w() as u32;
    let height = boundary.h() as u32;

    let background_texture = wgpu::TextureBuilder::new()
        .size([boundary.w() as u32, boundary.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.main_window().device());

    Model {
        tries: 0,
        rectangle_packer: rectangle_packer::RectanglePacker::new(app.window_rect()),
        background_image_buffer: image::ImageBuffer::from_fn(width, height, |x, y| {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 0;
            image::Rgba([r, g, b, 128])
        }),
        background_texture,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // if (app.elapsed_frames() % 60) == 0 {
    for _ in 0..500000 {
        model.rectangle_packer.add_random_rectangle();
    }
    model.tries += 1;
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    let draw = app.draw();

    // let sine = app.time.sin();
    // let slowersine = (app.time / 2.0).sin();

    let boundary = app.window_rect();

    // // Map the sine wave functions to ranges between the boundaries of the window
    // let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    // let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(PLUM);

    let background_flat_samples = model.background_image_buffer.as_flat_samples();
    model.background_texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &background_flat_samples.as_slice(),
    );

    let packer_debug_buffer = model.rectangle_packer.image_buffer();

    let packer_debug_texture = wgpu::TextureBuilder::new()
        .size([boundary.w() as u32, boundary.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.main_window().device());

    let packer_debug_flat_samples = packer_debug_buffer.as_flat_samples();

    packer_debug_texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &packer_debug_flat_samples.as_slice(),
    );

    draw.texture(&model.background_texture);
    // draw.texture(&packer_debug_texture);

    // draw.ellipse().color(STEELBLUE).x_y(x, y);

    for r in model.rectangle_packer.rectangles().iter() {
        draw.rect()
            .x_y(r.x, r.y)
            .w_h(r.width, r.height)
            // .color(rgba(0.0, 0.0, 25.0, 0.2))
            .color(ORANGERED)
            .stroke_color(YELLOW)
            .stroke_weight(1.0);
    }

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame as a png file
    // let file_path = captured_frame_path(app, &frame);
    // app.main_window().capture_frame(file_path);
}

/// Generate a path to save the given frame to.
#[allow(dead_code)]
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(format!("capture/{:03}", frame.nth()))
        .with_extension("png")
}
