use raqote::*;
use minifb::{Window, WindowOptions};
use std;

const LENGTH: usize = 1000;
const SIN_60: f32 = 0.86602540378;

struct Point {
    x: f32,
    y: f32,
}

fn dist(p0: &Point, p1: &Point) -> f32 {
    ((p0.x - p1.x) * (p0.x - p1.x) + (p0.y - p1.y) * (p0.y - p1.y)).sqrt()
}

fn translate(p: Point, delta: Point) -> Point {
    Point {
        x: p.x + delta.x,
        y: p.y + delta.y,
    }
}

fn scale(p: Point, s: f32) -> Point {
    Point {
        x: p.x * s,
        y: p.y * s,
    }
}

fn draw_koch_line(dt: &mut DrawTarget, window: &mut Window, p0: &Point, p1: &Point, depth: u32, max_depth: u32) {
    if max_depth <= depth {
        let mut pb = PathBuilder::new();
        pb.move_to(p0.x, p0.y);
        pb.line_to(p1.x, p1.y);
        let path = pb.finish();
        dt.stroke(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xFF, 0, 0x80, 0)),
            &StrokeStyle {
                width: 1.,
                ..StrokeStyle::default()
            },
            &DrawOptions::new(),
        );

        return;
    }
    let pa = Point {
        x: (2.0 * p0.x + p1.x) / 3.0,
        y: (2.0 * p0.y + p1.y) / 3.0,
    };

    let pb = Point {
        x: (2.0 * p1.x + p0.x) / 3.0,
        y: (2.0 * p1.y + p0.y) / 3.0,
    };

    let pm = Point {
        x: (p1.x + p0.x) / 2.0,
        y: (p1.y + p0.y) / 2.0,
    };

    let angle: f32 = (p1.y - p0.y).atan2(p1.x - p0.x) + (std::f32::consts::PI / 2.0);
    let d_3: f32 = dist(&pa, &pb);

    let pn = Point {
        x: pm.x + (d_3 * SIN_60 * angle.cos()),
        y: pm.y + (d_3 * SIN_60 * angle.sin()),
    };
    draw_koch_line(dt, window, &p0, &pa, depth + 1, max_depth);
    draw_koch_line(dt, window, &pa, &pn, depth + 1, max_depth);
    draw_koch_line(dt, window, &pn, &pb, depth + 1, max_depth);
    draw_koch_line(dt, window, &pb, &p1, depth + 1, max_depth);

    return ;

}

fn draw_koch_snowflake() {
    let vertical_padding = SIN_60 * LENGTH as f32 / 3.0;
    let win_width: usize = LENGTH;
    let win_height: usize = LENGTH;
    let mut window = Window::new(
        "Koch snowflake",
        win_width,
        win_height,
        WindowOptions {
            ..WindowOptions::default()
        }).unwrap();

    let mut pa: Point = Point { x: 0.0, y: 0.0};
    let mut pb: Point = Point { x: LENGTH as f32, y: 0.0};
    let mut pc: Point = Point { x: LENGTH as f32 / 2.0, y: LENGTH as f32 * SIN_60};

    let vert_scale = 1.0 / (SIN_60 * (4.0 / 3.0));
    let horiz_padding = (((1.0 / vert_scale) - 1.0) / 2.0) * LENGTH as f32;

    pa = translate(pa, Point{ x: horiz_padding, y: vertical_padding});
    pb = translate(pb, Point{ x: horiz_padding, y: vertical_padding});
    pc = translate(pc, Point{ x: horiz_padding, y: vertical_padding});

    pa = scale(pa, vert_scale);
    pb = scale(pb, vert_scale);
    pc = scale(pc, vert_scale);

    let mut dt = DrawTarget::new(win_width as i32, win_height as i32);

    for max_depth in 0..10 {
        dt = DrawTarget::new(win_width as i32, win_height as i32);
        draw_koch_line(
            &mut dt,
            &mut window,
            &pb,
            &pa,
            0,
            max_depth
        );
        draw_koch_line(
            &mut dt,
            &mut window,
            &pc,
            &pb,
            0,
            max_depth
        );
        draw_koch_line(
            &mut dt,
            &mut window,
            &pa,
            &pc,
            0,
            max_depth
        );


        window.update_with_buffer(dt.get_data(), LENGTH, LENGTH).unwrap();
        println!("Done with depth {}", max_depth);
        let sleep_time = std::time::Duration::from_millis(1000);
        std::thread::sleep(sleep_time);
    }
    loop {
        window.update_with_buffer(dt.get_data(), LENGTH, LENGTH).unwrap();
    }
}

fn main() {
    draw_koch_snowflake();
}

