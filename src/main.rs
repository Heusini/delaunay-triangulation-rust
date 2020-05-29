pub mod objects;

use crate::objects::point;

fn main() {
    let p1 = point::Point { x: 7.0, y: 8.0 };
    let p2 = point::Point { x: 10.0, y: 12.0 };

    println!("Ergebnis: {}", p1.dist(p2));
}
