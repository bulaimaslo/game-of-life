use macroquad::prelude::{coroutines::wait_seconds, *};

use crate::board::shapes::Shape;
mod board;

#[macroquad::main("GameOfLife")]
async fn main() {
    const SIZE: u16 = 40;
    let shape: Shape = Shape::GliderGun;
    let mut board = board::Board::new(SIZE);
    board.set_shape(shape);

    loop {
        clear_background(WHITE);

        for i in 0..SIZE * SIZE {
            let row = i / SIZE;
            let col = i % SIZE;
            let color = if board.get_cell_state_index(i as usize) {
                BLACK
            } else {
                WHITE
            };

            draw_rectangle(
                screen_width() / SIZE as f32 * col as f32,
                screen_height() / SIZE as f32 * row as f32,
                screen_width() / SIZE as f32,
                screen_height() / SIZE as f32,
                color,
            );
        }

        board = board.evaluate_next_board();

        if is_key_down(KeyCode::Space) {
            break;
        }

        wait_seconds(time_scale()).await;
        next_frame().await;
    }
}

fn time_scale() -> f32 {
    if is_key_down(KeyCode::Q) {
        0.1
    } else if is_key_down(KeyCode::W) {
        0.5
    } else if is_key_down(KeyCode::E) {
        1.0
    } else if is_key_down(KeyCode::R) {
        2.0
    } else if is_key_down(KeyCode::T) {
        5.0
    } else if is_key_down(KeyCode::Y) {
        10.0
    } else {
        0.0
    }
}
