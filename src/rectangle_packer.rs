// pub mod rectangle_packer {
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    /// Returns true if the given rectangle overlaps with this rectangle.
    pub fn is_overlap(&self, rect: &Rectangle) -> bool {
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

        let result = r.is_overlap(&Rectangle {
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

        let result = r.is_overlap(&Rectangle {
            x: 11.0,
            y: 11.0,
            width: 10.0,
            height: 10.0,
        });
        assert!(!result);
    }
}
// }
