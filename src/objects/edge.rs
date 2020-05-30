use crate::objects::point;
pub struct Edge {
    pub p1: point::Point,
    pub p2: point::Point,
}

impl Edge {
    pub fn get_length(&self) -> f64 {
        self.p1.dist(&self.p2)
    }

    pub fn get_middle_point(&self) -> point::Point {
        point::Point {
            x: (self.p2.x + self.p1.x) / 2.0,
            y: (self.p2.y + self.p1.y) / 2.0,
        }
    }
}
