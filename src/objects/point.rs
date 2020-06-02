use crate::numeric;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

const EPSILON: f64 = 0.0001;
impl Point {
    pub fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn distance_squared(&self, point: &Point) -> f64 {
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

    pub fn equal(&self, other: &Point) -> bool {
        if numeric::float_equal(self.x, other.x, EPSILON)
            && numeric::float_equal(self.y, other.y, EPSILON)
        {
            true
        } else {
            false
        }
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
