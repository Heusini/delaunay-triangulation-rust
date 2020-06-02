pub mod delaunay;
pub mod numeric;
pub mod objects;

use crate::objects::point;
use crate::objects::triangle;
use draw::*;
use rand::Rng;

fn convert_x(p1: f32) -> f32 {
    p1 + factor
}
fn convert_y(p1: f32) -> f32 {
    factor + (-1.0 * p1)
}
const size: u32 = 100;
const factor: f32 = size as f32 / 2.0;

fn draw_point(point: &point::Point, canvas: &mut draw::Canvas) {
    let rect = Drawing::new()
        .with_shape(Shape::Rectangle {
            width: 1,
            height: 1,
        })
        .with_xy(convert_x(point.x as f32), convert_y(point.y as f32))
        .with_style(Style::stroked(1, Color::random()));
    canvas.display_list.add(rect);
}

fn draw_triangle(triangle: &triangle::Triangle, canvas: &mut draw::Canvas) {
    let line = Drawing::new()
        .with_shape(
            LineBuilder::new(
                convert_x(triangle.p1.x as f32),
                convert_y(triangle.p1.y as f32),
            )
            .line_to(
                convert_x(triangle.p2.x as f32),
                convert_y(triangle.p2.y as f32),
            )
            .line_to(
                convert_x(triangle.p3.x as f32),
                convert_y(triangle.p3.y as f32),
            )
            .line_to(
                convert_x(triangle.p1.x as f32),
                convert_y(triangle.p1.y as f32),
            )
            .build(),
        )
        .with_style(Style::stroked(1, Color::random()));
    canvas.display_list.add(line);
}

fn draw_triangles(triangles: Vec<triangle::Triangle>, canvas: &mut draw::Canvas) {
    for tr in &triangles {
        //        draw_point(tr.p1, canvas);
        //        draw_point(tr.p2, canvas);
        //        draw_point(tr.p3, canvas);
        draw_triangle(tr, canvas);
    }
}

fn create_random_points() -> Vec<point::Point> {
    let mut points = Vec::with_capacity(100);
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let point = point::Point {
            x: rng.gen_range(-50.0, 50.0),
            y: rng.gen_range(-50.0, 50.0),
        };
        points.push(point);
    }
    points
}

fn main() {
    //    let p1 = point::Point { x: 7.0, y: 8.0 };
    //    let p2 = point::Point { x: 0.0, y: 12.0 };
    //    let p3 = point::Point { x: 0.0, y: 0.0 };
    //    let p4 = point::Point { x: 20.0, y: -4.0 };
    //    let p5 = point::Point { x: -20.0, y: 4.0 };
    //
    //    let points: Vec<point::Point> = vec![p5, p4, p3, p2, p1];

    let points = create_random_points();

    let triangles: Vec<triangle::Triangle> = delaunay::triangulate(&points);

    let mut canvas = Canvas::new(size, size);
    for p in &points {
        draw_point(p, &mut canvas);
        println!("{:?}", p);
    }

    draw_triangles(triangles, &mut canvas);

    // save the canvas as an svg
    render::save(&canvas, "tests/basic_end_to_end.svg", SvgRenderer::new())
        .expect("Failed to save");
}
