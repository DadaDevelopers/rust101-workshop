#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Creates a new Rectangle.
    /// TODO: implement this function.
    pub fn new(width: f64, height: f64) -> Self {
        todo!()
    }

    /// Returns the area of the rectangle.
    /// TODO: implement this function.
    pub fn area(&self) -> f64 {
        todo!()
    }

    /// Returns true if this rectangle can contain `other` (strictly).
    /// TODO: implement this function.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }
}
