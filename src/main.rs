use nannou::prelude::*;

mod rectangle_packer;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1000, 1000)
        // .loop_mode(LoopMode::NTimes {
        //     number_of_updates: 1,
        // })
        .run();
}

struct Model {
    tries: u32,
    rects: Vec<rectangle_packer::Rectangle>,
}

fn model(_app: &App) -> Model {
    let test_rects: Vec<rectangle_packer::Rectangle> = Vec::new();
    Model {
        tries: 0,
        rects: test_rects,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let boundary = app.window_rect();

    // if (app.elapsed_frames() % 120) == 0 {
    let new_rect = rectangle_packer::Rectangle {
        x: random_range(boundary.left() / 1.1, boundary.right() / 1.1),
        y: random_range(boundary.bottom() / 1.1, boundary.top() / 1.1),
        width: random_range(4.0, 30.0),
        height: random_range(4.0, 30.0),
    };
    model.tries += 1;

    let mut is_overlap = false;
    for r in model.rects.iter() {
        if r.is_overlap(&new_rect) {
            is_overlap = true;
            break;
        }
    }

    if !is_overlap {
        model.rects.push(new_rect);
        println!(
            "Rect count is: {} and tries are:{} frames:{}",
            model.rects.len(),
            model.tries,
            app.elapsed_frames(),
        );
    }
    // }
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

    for r in model.rects.iter() {
        draw.rect()
            .x_y(r.x, r.y)
            .w_h(r.width, r.height)
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
