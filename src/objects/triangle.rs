use crate::objects::edge;
use crate::objects::point;

pub struct Triangle {
    pub p1: point::Point,
    pub p2: point::Point,
    pub p3: point::Point,
    pub is_bad: bool,
    pub e1: edge::Edge,
    pub e2: edge::Edge,
    pub e3: edge::Edge,
}

impl Triangle {
    pub fn new(p1: point::Point, p2: point::Point, p3: point::Point) -> Triangle {
        Triangle {
            p1,
            p2,
            p3,
            is_bad: false,
            e1: edge::Edge { p1, p2 },
            e2: edge::Edge { p1: p2, p2: p3 },
            e3: edge::Edge { p1: p3, p2: p1 },
        }
    }
    pub fn circum_circle_contains(&self, p: &point::Point) -> bool {
        let ab: f64 = self.p1.norm2();
        let cd: f64 = self.p2.norm2();
        let ef: f64 = self.p3.norm2();

        let cirum_x = (ab * (self.p3.y - self.p2.y)
            + cd * (self.p1.y - self.p3.y)
            + ef * (self.p2.y - self.p1.y))
            / (self.p1.x * (self.p3.y - self.p2.y)
                + self.p2.x * (self.p1.y - self.p3.y)
                + self.p3.x * (self.p2.y - self.p1.y));

        let circum_y = (ab * (self.p3.x - self.p2.x)
            + cd * (self.p1.x - self.p3.x)
            + ef * (self.p2.x - self.p1.x))
            / (self.p1.y * (self.p3.x - self.p2.x)
                + self.p2.y * (self.p1.x - self.p3.x)
                + self.p3.y * (self.p2.x - self.p1.x));

        let circum: point::Point = point::Point {
            x: cirum_x * 0.5,
            y: circum_y * 0.5,
        };

        let circum_radius = self.p1.distance_squared(&circum);
        let dist = p.distance_squared(&circum);
        dist <= circum_radius
    }

    pub fn contains_vertex(&self, p: &point::Point) -> bool {
        self.p1.equal(p) || self.p2.equal(p) || self.p3.equal(p)
    }
}
