use nannou::rand::Rng;
use nannou::{image, rand::rngs::SmallRng};

// actual color not important since we're just checking for overlap
const RECT_STROKE_COLOR: image::Rgba<u8> = image::Rgba([0, 255, 0, 255]);
const RECT_FILL_COLOR: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

pub struct RectanglePacker {
    pub boundary: nannou::geom::Rect,
    rectangles: Vec<Rectangle>,
    image_buffer: nannou::image::RgbaImage,
    // pub background_image: Option<image::RgbaImage>,
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
        let new_buffer = image::ImageBuffer::from_pixel(width, height, image::Rgba([0, 0, 0, 255]));

        Self {
            boundary,
            rectangles: Vec::new(),
            image_buffer: new_buffer,
        }
    }

    pub fn rectangles(&self) -> &Vec<Rectangle> {
        &self.rectangles
    }

    pub fn image_buffer(&self) -> &nannou::image::RgbaImage {
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
        image_buffer: &mut nannou::image::RgbaImage,
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
        image_buffer: &nannou::image::RgbaImage,
        padding_around_rectangle: u32,
    ) -> bool {
        let center = self.center_from_nannou_rect(boundary);

        let left = center.0 as u32 - (self.width / 2.0) as u32 - padding_around_rectangle;
        let right = center.0 as u32 + (self.width / 2.0) as u32 + padding_around_rectangle;

        let top = center.1 as u32 - (self.height / 2.0) as u32 - padding_around_rectangle;
        let bottom = center.1 as u32 + (self.height / 2.0) as u32 + padding_around_rectangle;

        let mut is_open = true;
        let initial_pixel_color = image_buffer.get_pixel(left, top);
        for x in left..right + 1 {
            for y in top..bottom + 1 {
                if image_buffer.get_pixel(x, y) != initial_pixel_color {
                    is_open = false;
                    break;
                }
            }
        }
        is_open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn does_overlap() {
    //     let r = Rectangle {
    //         x: 0.0,
    //         y: 0.0,
    //         width: 10.0,
    //         height: 10.0,
    //     };

    //     let result = r.is_overlap(&Rectangle {
    //         x: 5.0,
    //         y: 5.0,
    //         width: 10.0,
    //         height: 10.0,
    //     });
    //     assert!(result);
    // }

    // #[test]
    // fn does_not_overlap() {
    //     let r = super::Rectangle {
    //         x: 0.0,
    //         y: 0.0,
    //         width: 10.0,
    //         height: 10.0,
    //     };

    //     let result = r.is_overlap(&Rectangle {
    //         x: 11.0,
    //         y: 11.0,
    //         width: 10.0,
    //         height: 10.0,
    //     });
    //     assert!(!result);
    // }

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
