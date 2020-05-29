use std::ops;
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn dist(self, other: Point) -> f64 {
        (other.x - self.x).hypot(other.y - self.y)
    }

    pub fn crossproduct(self, other: Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn scalar(self, other: Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
