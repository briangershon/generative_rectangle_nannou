use nannou::{prelude::*, rand::rngs::SmallRng, LoopMode};
mod rectangle_packer;
use first_nannou_project::texture_from_image_buffer;
use nannou::rand;
use nannou::rand::SeedableRng;

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
    rectangle_packer: rectangle_packer::RectanglePacker,
    background_image_buffer: imageproc::image::ImageBuffer<imageproc::image::Rgba<u8>, Vec<u8>>,
    rng: SmallRng,
}

fn model(app: &App) -> Model {
    // For seeded randomness
    // let seed: [u8; 32] = [
    //     10, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    //     26, 27, 28, 29, 30, 31, 32,
    // ];
    // let rng = SmallRng::from_seed(seed);

    // for general randomness
    let rng = SmallRng::from_rng(rand::thread_rng()).unwrap();

    let boundary = app.window_rect();
    let width = boundary.w() as u32;
    let height = boundary.h() as u32;

    Model {
        rectangle_packer: rectangle_packer::RectanglePacker::new(app.window_rect()),
        background_image_buffer: imageproc::image::ImageBuffer::from_fn(width, height, |x, y| {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 0;
            imageproc::image::Rgba([r, g, b, 128])
        }),
        rng,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _ in 0..500000 {
        model.rectangle_packer.add_random_rectangle(&mut model.rng);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    let draw = app.draw();

    draw.background().color(PLUM);
    let background_texture = texture_from_image_buffer(app, &frame, &model.background_image_buffer);
    draw.texture(&background_texture);

    // uncomment following lines to see the rectangle packer buffer for debugging
    // let packer_debug_buffer = model.rectangle_packer.image_buffer();
    // let packer_debug_texture = texture_from_image_buffer(app, &frame, &packer_debug_buffer);
    // draw.texture(&packer_debug_texture);

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
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

/// Generate a path to save the given frame to.
#[allow(dead_code)]
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(format!("capture/{:03}", frame.nth()))
        .with_extension("png")
}
