use macroquad::prelude::*;

fn rand_uniform() -> f32 {
    rand::rand() as f32 / (u32::MAX as f32)
}

#[macroquad::main("Thermo")]
async fn main() {
    loop {
        clear_background(BLACK);

        for _ in 0..255 {
            let x = rand_uniform() * screen_width();
            let y = rand_uniform() * screen_height();

            draw_circle(x, y, 15.0 * rand_uniform(), YELLOW);
        }

        next_frame().await
    }
}
