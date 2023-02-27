use macroquad::prelude::*;

const SIZE: u16 = 20;

#[macroquad::main("GameOfLife")]
async fn main() {
    let mut board = [false; (SIZE * SIZE) as usize];

    board[0] = true;
    board[21] = true;
    board[42] = true;
    board[63] = true;
    board[84] = true;

    loop {
        clear_background(RED);

        for i in 0..SIZE * SIZE {
            let row = i / SIZE;
            let col = i % SIZE;

            draw_rectangle(
                screen_width() / SIZE as f32 * col as f32,
                screen_height() / SIZE as f32 * row as f32,
                screen_width() / SIZE as f32,
                screen_height() / SIZE as f32,
                if board[i as usize] { BLACK } else { WHITE },
            );
        }

        if is_key_down(KeyCode::Space) {
            break;
        }

        next_frame().await
    }
}

fn get_cell_state(board: &[bool], row: u16, col: u16) -> bool {
    if col >= SIZE || row >= SIZE {
        panic!("Out of bounds");
    }

    board[(row * SIZE + col) as usize]
}
