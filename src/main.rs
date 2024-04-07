use nannou::prelude::*;
mod rectangle_packer;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1000, 1000)
        .run();
}

struct Model {
    tries: u32,
    rectangle_packer: rectangle_packer::RectanglePacker,
}

fn model(app: &App) -> Model {
    Model {
        tries: 0,
        rectangle_packer: rectangle_packer::RectanglePacker::new(app.window_rect()),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if (app.elapsed_frames() % 60) == 0 {
        model.rectangle_packer.add_random_rectangle();
        model.tries += 1;
        println!(
            "Rect count is: {}",
            model.rectangle_packer.rectangles().len()
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    let draw = app.draw();

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    draw.background().color(PLUM);

    // Draw a blue ellipse with a radius of 10 at the (x,y) coordinates of (0.0, 0.0)
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    for r in model.rectangle_packer.rectangles().iter() {
        draw.rect()
            .x_y(r.x, r.y)
            .w_h(r.width, r.height)
            .color(ORANGERED)
            .stroke_color(YELLOW)
            .stroke_weight(1.0);
    }

    let image_buffer = model.rectangle_packer.image_buffer();

    let texture = wgpu::TextureBuilder::new()
        .size([boundary.w() as u32, boundary.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(app.main_window().device());

    let flat_samples = image_buffer.as_flat_samples();
    texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );

    draw.texture(&texture);

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
