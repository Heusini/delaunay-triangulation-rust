use std::ops;
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn pytagoras(&self, point: &Point) -> f64 {
        let dx = self.x - point.x;
        let dy = self.y - point.y;
        dx * dx + dy * dy
    }

    pub fn dist(&self, other: &Point) -> f64 {
        (other.x - self.x).hypot(other.y - self.y)
    }

    pub fn crossproduct(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn scalar(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl ops::Add<Point> for &Point {
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
