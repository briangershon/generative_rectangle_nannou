use nannou::{image, rand::random_range};

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
        let new_buffer = image::ImageBuffer::from_fn(width, height, |x, y| {
            let r = (x as f32 / width as f32 * 255.0) as u8;
            let g = (y as f32 / height as f32 * 255.0) as u8;
            let b = 0;
            image::Rgba([r, g, b, 128])
        });

        // new_buffer.put_pixel(10, 10, image::Rgba([255, 0, 0, 255]));
        // new_buffer.put_pixel(11, 10, image::Rgba([255, 0, 0, 255]));
        // new_buffer.put_pixel(12, 10, image::Rgba([255, 0, 0, 255]));

        // println!("Image buffer 10, 10: {:?}", new_buffer.get_pixel(10, 10));

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

    pub fn add_random_rectangle(&mut self) {
        let new_rect = Rectangle {
            x: random_range(self.boundary.left() / 1.1, self.boundary.right() / 1.1),
            y: random_range(self.boundary.bottom() / 1.1, self.boundary.top() / 1.1),
            width: random_range(4.0, 60.0),
            height: random_range(4.0, 60.0),
        };

        println!(
            "plotting rectangle: {} {} {} {}",
            new_rect.x, new_rect.y, new_rect.width, new_rect.height
        );

        new_rect.draw_rect_on_buffer(self.boundary, &mut self.image_buffer);

        self.rectangles.push(new_rect);
    }
}

impl Rectangle {
    /// Returns true if the given rectangle overlaps with this rectangle.
    // pub fn is_overlap(&self, rect: &Rectangle) -> bool {
    //     let x_overlap = self.x + self.width > rect.x && rect.x + rect.width > self.x;
    //     let y_overlap = self.y + self.height > rect.y && rect.y + rect.height > self.y;
    //     x_overlap && y_overlap
    // }

    fn draw_rect_on_buffer(
        &self,
        boundary: nannou::geom::Rect,
        image_buffer: &mut nannou::image::RgbaImage,
    ) {
        // let buffer = &mut self.image_buffer;

        let origin_x = boundary.w() as u32 / 2;
        let origin_y = boundary.h() as u32 / 2;
        for x in self.x as u32..(self.x + self.width) as u32 {
            for y in self.y as u32..(self.y + self.height) as u32 {
                // let rect_center_x = new_rect.x + new_rect.width / 2.0;
                // let rect_center_t = new_rect.y + new_rect.width / 2.0;
                image_buffer.put_pixel(
                    (origin_x) + x - self.width as u32 / 2 - 1,
                    (origin_y) - y + self.height as u32 / 2 - 1,
                    image::Rgba([255, 0, 0, 255]),
                );
            }
        }
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
}
