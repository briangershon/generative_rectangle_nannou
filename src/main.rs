use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view).size(200, 200)
        .loop_mode(LoopMode::NTimes {number_of_updates:1})
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);

    let draw = app.draw();

    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    let boundary = app.window_rect();

    // Map the sine wave functions to ranges between the boundaries of the window
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Clear the background to purple.
    draw.background().color(PLUM);

    // Draw a blue ellipse with a radius of 10 at the (x,y) coordinates of (0.0, 0.0)
    draw.ellipse().color(STEELBLUE).x_y(x,y);

    draw.to_frame(app, &frame).unwrap();

    // // Capture the frame as a png file
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

/// Generate a path to save the given frame to.
fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(format!("capture/{:03}", frame.nth()))
        .with_extension("png")
}