pub mod numeric;
pub mod objects;

struct Delauny {
    triangles: Vec<objects::triangle::Triangle>,
    edges: Vec<objects::edge::Edge>,
    points: Vec<objects::point::Point>,
}

#[test]
fn test_empty_pointlist() {
    let pointlist = Vec::<objects::point::Point>::new();
    let triangles = triangulate(&pointlist);

    assert_eq!(triangles.len(), 0);
}

pub fn triangulate(vertices: &Vec<objects::point::Point>) -> Vec<objects::triangle::Triangle> {
    let mut triangles: Vec<objects::triangle::Triangle> = Vec::new();
    if vertices.len() < 3 {
        return triangles;
    }

    let mut min_x = vertices[0].x;
    let mut min_y = vertices[0].y;

    let mut max_x = min_x;
    let mut max_y = min_y;

    for val in vertices {
        if val.x < min_x {
            min_x = val.x;
        }

        if val.y < min_y {
            min_y = val.y;
        }

        if val.x > max_x {
            max_x = val.x;
        }

        if val.y > max_y {
            max_y = val.y;
        }
    }

    let dx = max_x - min_x;
    let dy = max_y - min_y;

    let delta_max = f64::max(dx, dy);
    let mid_x = (min_x + max_x) * 0.5;
    let mid_y = (min_y + max_y) * 0.5;

    let p1 = objects::point::Point {
        x: mid_x - 20.0 * delta_max,
        y: mid_y - delta_max,
    };

    let p2 = objects::point::Point {
        x: mid_x,
        y: mid_y + 20.0 * delta_max,
    };

    let p3 = objects::point::Point {
        x: mid_x + 20.0 * delta_max,
        y: mid_y - delta_max,
    };

    triangles.push(objects::triangle::Triangle::new(p1, p2, p3));

    for vert in vertices {
        let mut edges: Vec<objects::edge::Edge> = Vec::new();

        triangles.retain(|triangle| {
            if triangle.circum_circle_contains(vert) {
                edges.push(triangle.e1);
                edges.push(triangle.e2);
                edges.push(triangle.e3);
                false
            } else {
                true
            }
        });

        let mut bad_edges: Vec<bool> = vec![true; edges.len()];
        let mut outer: usize = 0;
        for e1 in &edges {
            let mut inner: usize = outer + 1;
            for e2 in &edges[inner..] {
                if e1.equal(e2) {
                    bad_edges[outer] = false;
                    bad_edges[inner] = false;
                }
                inner += 1;
            }
            outer += 1;
        }

        let mut i = 0;
        edges.retain(|_| (bad_edges[i], i += 1).0);

        for edge in &edges {
            triangles.push(objects::triangle::Triangle::new(edge.p1, edge.p2, *vert));
        }
    }

    triangles.retain(|triangle| {
        !triangle.contains_vertex(&p1)
            && !triangle.contains_vertex(&p2)
            && !triangle.contains_vertex(&p3)
    });

    triangles
}
