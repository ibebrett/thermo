use macroquad::prelude::*;

fn rand_uniform() -> f32 {
    rand::rand() as f32 / (u32::MAX as f32)
}

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy)]
struct Line {
    p1: Point,
    p2: Point,
}

fn update_lines(lines: &mut Vec<Line>, num_lines: usize) {
    lines.clear();

    let step_size = 10.0;

    // Directions
    // 0 - right
    // 1 - up
    // 2 - left
    // 3 - down
    let mut last_dir = 0;

    // Lets start with a line in the middle
    lines.push(Line {
        p1: Point { x: 500.0, y: 500.0 },
        p2: Point {
            x: 500.0 + step_size,
            y: 500.0,
        },
    });

    let mut last_vec = &lines[0];

    for _ in 0..num_lines {
        // Pick a direction, 90 degrees left or right.
        let dir = (last_dir + (if rand_uniform() > 0.5 { -1 } else { 1 }) + 4) % 4;
        let max_length = rand_uniform() * step_size;

        let last_point = &last_vec.p2;

        // Create a new point.
        let new_point = match dir {
            0 => Point {
                x: last_point.x + max_length,
                y: last_point.y,
            },
            1 => Point {
                x: last_point.x,
                y: last_point.y + max_length,
            },
            2 => Point {
                x: last_point.x - max_length,
                y: last_point.y,
            },
            _ => Point {
                x: last_point.x,
                y: last_point.y - max_length,
            },
        };

        // Generate a new line.
        let l = Line {
            p1: last_point.clone(),
            p2: new_point,
        };

        last_dir = dir;
        lines.push(l);
        last_vec = &lines[lines.len() - 1];
    }
}

#[macroquad::main("Thermo")]
async fn main() {
    let num_lines = 100000;
    let mut lines = Vec::with_capacity(num_lines);

    update_lines(&mut lines, num_lines);

    let mut center = Point { x: 500.0, y: 500.0 };

    let mut cam = Camera2D {
        zoom: Vec2::new(1.0 / screen_width(), 1.0 / screen_height()),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        let mut reset = false;
        reset |= is_key_pressed(KeyCode::R);

        if is_key_down(KeyCode::Right) {
            center.x += 10.0;
        }
        if is_key_down(KeyCode::Left) {
            center.x -= 10.0;
        }
        if is_key_down(KeyCode::Down) {
            center.y -= 10.0;
        }
        if is_key_down(KeyCode::Up) {
            center.y += 10.0;
        }

        cam.target.x = center.x;
        cam.target.y = center.y;
        set_camera(&cam);

        if reset {
            update_lines(&mut lines, num_lines);
        }

        for l in &lines {
            draw_line(l.p1.x, l.p1.y, l.p2.x, l.p2.y, 1.0, YELLOW);
        }

        next_frame().await
    }
}
