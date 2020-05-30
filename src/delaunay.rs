use crate::objects::edge;
use crate::objects::point;
use crate::objects::triangle;

struct Delauny {
    triangles: Vec<triangle::Triangle>,
    edges: Vec<edge::Edge>,
    points: Vec<point::Point>,
}

fn triangulate(vertices: Vec<point::Point>) -> Vec<triangle::Triangle> {
    let minX = vertices[0].x;
    let minY = vertices[0].y;

    let mut maxX = minX;
    let mut maxY = minY;

    //    for val in vertices.iter() {}
    vec![]
}
