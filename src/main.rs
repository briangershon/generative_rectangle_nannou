use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(200, 200)
        .loop_mode(LoopMode::NTimes {
            number_of_updates: 1,
        })
        .run();
}

struct Model {
    rects: [Rectangle; 2],
}

fn model(_app: &App) -> Model {
    Model {
        rects: [
            Rectangle {
                x: -20.0,
                y: -20.0,
                width: 10.0,
                height: 10.0,
            },
            Rectangle {
                x: 40.0,
                y: 40.0,
                width: 100.0,
                height: 100.0,
            },
        ],
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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
            .color(ORANGERED);
    }

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame as a png file
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

struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Rectangle {
    /// Returns true if the given rectangle overlaps with this rectangle.
    fn is_overlap(self, rect: Rectangle) -> bool {
        let x_overlap = self.x + self.width > rect.x && rect.x + rect.width > self.x;
        let y_overlap = self.y + self.height > rect.y && rect.y + rect.height > self.y;
        x_overlap && y_overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_overlap() {
        let r = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };

        let result = r.is_overlap(Rectangle {
            x: 5.0,
            y: 5.0,
            width: 10.0,
            height: 10.0,
        });
        assert!(result);
    }

    #[test]
    fn does_not_overlap() {
        let r = super::Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };

        let result = r.is_overlap(super::Rectangle {
            x: 11.0,
            y: 11.0,
            width: 10.0,
            height: 10.0,
        });
        assert!(!result);
    }
}
