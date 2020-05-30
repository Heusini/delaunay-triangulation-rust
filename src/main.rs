pub mod delaunay;
pub mod objects;

use crate::objects::point;
use crate::objects::triangle;
use draw::*;

fn convert_x(p1: f32, factor: f32) -> f32 {
    p1 + factor
}
fn convert_y(p1: f32, factor: f32) -> f32 {
    factor + (-1.0 * p1)
}

fn main() {
    let size: u32 = 100;
    let factor: f32 = size as f32 / 2.0;
    let p1 = point::Point { x: 7.0, y: 8.0 };
    let p2 = point::Point { x: 0.0, y: 12.0 };
    let p3 = point::Point { x: 0.0, y: 0.0 };

    println!("Ergebnis: {}", p1.dist(&p2));

    let triangle = triangle::Triangle { p1, p2, p3 };
    let mut canvas = Canvas::new(size, size);

    let rect2 = Drawing::new()
        .with_shape(Shape::Rectangle {
            width: 1,
            height: 1,
        })
        .with_xy(
            convert_x(triangle.p1.x as f32, factor),
            convert_y(triangle.p1.y as f32, factor),
        )
        .with_style(Style::stroked(1, Color::random()));
    let rect3 = Drawing::new()
        .with_shape(Shape::Rectangle {
            width: 1,
            height: 1,
        })
        .with_xy(
            convert_x(triangle.p2.x as f32, factor),
            convert_y(triangle.p2.y as f32, factor),
        )
        .with_style(Style::stroked(1, Color::random()));
    let rect4 = Drawing::new()
        .with_shape(Shape::Rectangle {
            width: 1,
            height: 1,
        })
        .with_xy(
            convert_x(triangle.p3.x as f32, factor),
            convert_y(triangle.p3.y as f32, factor),
        )
        .with_style(Style::stroked(1, Color::random()));

    let y_line = Drawing::new()
        .with_shape(
            LineBuilder::new(factor, 0.0)
                .line_to(factor, size as f32)
                .build(),
        )
        .with_style(Style::stroked(1, Color::black()));
    let x_line = Drawing::new()
        .with_shape(
            LineBuilder::new(0.0, factor)
                .line_to(size as f32, factor)
                .build(),
        )
        .with_style(Style::stroked(1, Color::black()));

    let line1 = Drawing::new()
        .with_shape(
            LineBuilder::new(
                convert_x(triangle.p1.x as f32, factor),
                convert_y(triangle.p1.y as f32, factor),
            )
            .line_to(
                convert_x(triangle.p2.x as f32, factor),
                convert_y(triangle.p2.y as f32, factor),
            )
            .line_to(
                convert_x(triangle.p3.x as f32, factor),
                convert_y(triangle.p3.y as f32, factor),
            )
            .line_to(
                convert_x(triangle.p1.x as f32, factor),
                convert_y(triangle.p1.y as f32, factor),
            )
            .build(),
        )
        .with_style(Style::stroked(1, Color::random()));

    // add it to the canvas
    canvas.display_list.add(rect2);
    canvas.display_list.add(rect4);
    canvas.display_list.add(rect3);
    canvas.display_list.add(y_line);
    canvas.display_list.add(x_line);
    canvas.display_list.add(line1);

    // save the canvas as an svg
    render::save(&canvas, "tests/basic_end_to_end.svg", SvgRenderer::new())
        .expect("Failed to save");
}
