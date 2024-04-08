// use ab_glyph::{FontRef, PxScale};
use imageproc::drawing::draw_filled_ellipse_mut;
// use imageproc::drawing::draw_text_mut;
// use nannou::image;
use nannou::rand::rngs::SmallRng;
use nannou::rand::Rng;

// actual color not important since we're just checking for overlap
const RECT_STROKE_COLOR: imageproc::image::Rgba<u8> = imageproc::image::Rgba([0, 255, 0, 255]);
const RECT_FILL_COLOR: imageproc::image::Rgba<u8> = imageproc::image::Rgba([255, 0, 0, 255]);
const RECT_HIDE_COLOR: imageproc::image::Rgba<u8> = imageproc::image::Rgba([0, 255, 0, 255]);

pub struct RectanglePacker {
    pub boundary: nannou::geom::Rect,
    rectangles: Vec<Rectangle>,
    image_buffer: imageproc::image::RgbaImage,
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectanglePacker {
    pub fn new(boundary: nannou::geom::Rect) -> Self {
        let width = boundary.w() as u32;
        let height = boundary.h() as u32;
        let new_buffer = imageproc::image::ImageBuffer::from_pixel(
            width,
            height,
            imageproc::image::Rgba([0, 0, 0, 255]),
        );

        let mut imageproc_buffer =
            imageproc::image::ImageBuffer::from_raw(width, height, new_buffer.into_raw())
                .expect("Failed to convert image buffer");

        draw_filled_ellipse_mut(
            &mut imageproc_buffer,
            (width as i32 / 2, height as i32 / 2),
            (boundary.w() * 0.4) as i32,
            (boundary.h() * 0.4) as i32,
            RECT_HIDE_COLOR,
        );

        draw_filled_ellipse_mut(
            &mut imageproc_buffer,
            (width as i32 / 2, height as i32 / 2),
            (boundary.w() * 0.4 - 40.0) as i32,
            (boundary.h() * 0.4 - 40.0) as i32,
            imageproc::image::Rgba([0, 0, 0, 255]),
        );

        // // "/Library/Fonts/Arial Unicode.ttf"
        // // "/System/Library/Fonts/MarkerFelt.ttc"
        // let font = FontRef::try_from_slice(include_bytes!("/Library/Fonts/Arial Unicode.ttf"))
        //     .expect("could not load font");

        // let font_size = 1000.0;
        // let scale = PxScale {
        //     x: font_size,
        //     y: font_size,
        // };

        // draw_text_mut(
        //     &mut imageproc_buffer,
        //     RECT_HIDE_COLOR,
        //     width as i32 / 2 - 270,
        //     height as i32 / 2 - 525,
        //     scale,
        //     &font,
        //     "B",
        // );

        Self {
            boundary,
            rectangles: Vec::new(),
            image_buffer: imageproc_buffer,
        }
    }

    pub fn rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }

    #[allow(dead_code)]
    pub fn image_buffer(&self) -> &imageproc::image::RgbaImage {
        &self.image_buffer
    }

    pub fn add_random_rectangle(&mut self, rng: &mut SmallRng) {
        let new_rect = Rectangle {
            x: rng.gen_range(self.boundary.left() / 1.1..self.boundary.right() / 1.1),
            y: rng.gen_range(self.boundary.bottom() / 1.1..self.boundary.top() / 1.1),
            width: rng.gen_range(4.0..60.0),
            height: rng.gen_range(4.0..60.0),
        };

        if new_rect.open_rect_on_buffer(self.boundary, &self.image_buffer, 4) {
            new_rect.draw_rect_on_buffer(self.boundary, &mut self.image_buffer);
            self.rectangles.push(new_rect);
        }
    }
}

impl Rectangle {
    fn center_from_nannou_rect(&self, boundary: nannou::geom::Rect) -> (f32, f32) {
        let origin_x = boundary.w() / 2.0;
        let origin_y = boundary.h() / 2.0;

        (origin_x + self.x, origin_y - self.y)
    }

    fn draw_rect_on_buffer(
        &self,
        boundary: nannou::geom::Rect,
        image_buffer: &mut imageproc::image::RgbaImage,
    ) {
        let center = self.center_from_nannou_rect(boundary);

        let left = center.0 as u32 - (self.width / 2.0) as u32;
        let right = center.0 as u32 + (self.width / 2.0) as u32;

        let top = center.1 as u32 - (self.height / 2.0) as u32;
        let bottom = center.1 as u32 + (self.height / 2.0) as u32;

        for x in left..right + 1 {
            for y in top..bottom + 1 {
                let mut rect_color = RECT_FILL_COLOR;
                if (x == left || x == right) || (y == top || y == bottom) {
                    rect_color = RECT_STROKE_COLOR;
                }
                image_buffer.put_pixel(x, y, rect_color);
            }
        }
    }

    fn open_rect_on_buffer(
        &self,
        boundary: nannou::geom::Rect,
        image_buffer: &imageproc::image::RgbaImage,
        padding_around_rectangle: u32,
    ) -> bool {
        let center = self.center_from_nannou_rect(boundary);
        let left = center.0 as u32 - (self.width / 2.0) as u32 - padding_around_rectangle;
        let right = center.0 as u32 + (self.width / 2.0) as u32 + padding_around_rectangle;
        let top = center.1 as u32 - (self.height / 2.0) as u32 - padding_around_rectangle;
        let bottom = center.1 as u32 + (self.height / 2.0) as u32 + padding_around_rectangle;

        let mut is_open = true;

        let initial_pixel_color = image_buffer.get_pixel(left, top);
        if initial_pixel_color.eq(&RECT_HIDE_COLOR) {
            is_open = false;
        } else {
            for x in left..right + 1 {
                for y in top..bottom + 1 {
                    let current_color = image_buffer.get_pixel(x, y);

                    if current_color.eq(&RECT_HIDE_COLOR) {
                        is_open = false;
                        break;
                    }
                    if image_buffer.get_pixel(x, y).ne(initial_pixel_color) {
                        is_open = false;
                        break;
                    }
                }
            }
        }
        is_open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_from_origin() {
        let r = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };

        let result = r.center_from_nannou_rect(nannou::geom::Rect::from_w_h(1000.0, 1000.0));
        assert_eq!(result, (500.0, 500.0));
    }

    #[test]
    fn center_from_bottom_right() {
        let r = Rectangle {
            x: 200.0,
            y: -100.0,
            width: 10.0,
            height: 10.0,
        };

        let result = r.center_from_nannou_rect(nannou::geom::Rect::from_w_h(1000.0, 1000.0));
        assert_eq!(result, (700.0, 600.0));
    }
}
