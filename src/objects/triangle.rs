use crate::objects::point;
pub struct Triangle {
    pub p1: point::Point,
    pub p2: point::Point,
    pub p3: point::Point,
}

impl Triangle {
    fn circum_circle_contains(&self, p: point::Point) -> bool {
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

        let circum_radius = self.p1.pytagoras(&circum);
        let dist = p.pytagoras(&circum);
        dist <= circum_radius
    }
}
