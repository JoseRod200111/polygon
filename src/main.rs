use raylib::prelude::*;

fn put_pixel(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    d.draw_pixel(x, y, color);
}

fn draw_line(
    d: &mut RaylibDrawHandle,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        put_pixel(d, x0, y0, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let error_doble = 2 * error;

        if error_doble >= dy {
            error += dy;
            x0 += sx;
        }

        if error_doble <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

fn draw_polygon(
    d: &mut RaylibDrawHandle,
    points: &[(i32, i32)],
    color: Color,
) {
    for i in 0..points.len() {
        let current = points[i];
        let next = points[(i + 1) % points.len()];

        draw_line(
            d,
            current.0,
            current.1,
            next.0,
            next.1,
            color,
        );
    }
}

fn fill_polygon(
    d: &mut RaylibDrawHandle,
    points: &[(i32, i32)],
    color: Color,
) {
    let min_y = points
        .iter()
        .map(|point| point.1)
        .min()
        .unwrap();

    let max_y = points
        .iter()
        .map(|point| point.1)
        .max()
        .unwrap();

    for y in min_y..=max_y {
        let mut intersections: Vec<i32> = Vec::new();

        for i in 0..points.len() {
            let current = points[i];
            let next = points[(i + 1) % points.len()];

            let x1 = current.0;
            let y1 = current.1;
            let x2 = next.0;
            let y2 = next.1;

            if y1 == y2 {
                continue;
            }

            let crosses_scanline =
                (y1 <= y && y < y2) ||
                (y2 <= y && y < y1);

            if crosses_scanline {
                let x = x1 as f32
                    + (y - y1) as f32
                        * (x2 - x1) as f32
                        / (y2 - y1) as f32;

                intersections.push(x.round() as i32);
            }
        }

        intersections.sort();

        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                for x in pair[0]..=pair[1] {
                    put_pixel(d, x, y, color);
                }
            }
        }
    }
}

fn main() {
    let polygon_3 = [
        (377, 249),
        (411, 197),
        (436, 249),
    ];

    let (mut rl, thread) = raylib::init()
        .size(800, 500)
        .title("Poligono 3")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        fill_polygon(
            &mut d,
            &polygon_3,
            Color::RED,
        );

        draw_polygon(
            &mut d,
            &polygon_3,
            Color::WHITE,
        );
    }
}