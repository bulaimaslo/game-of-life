use macroquad::prelude::{coroutines::wait_seconds, *};

const SIZE: u16 = 40;

#[macroquad::main("GameOfLife")]
async fn main() {
    let mut board = [false; (SIZE * SIZE) as usize];

    set_gosper_glider_gun(&mut board);


    loop {
        clear_background(WHITE);

        for i in 0..SIZE * SIZE {
            let row = i / SIZE;
            let col = i % SIZE;
            let color = if board[i as usize] { BLACK } else { WHITE };

            draw_rectangle(
                screen_width() / SIZE as f32 * col as f32,
                screen_height() / SIZE as f32 * row as f32,
                screen_width() / SIZE as f32,
                screen_height() / SIZE as f32,
                color,
            );
        }

        board = evaluate_next_board(&board);

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

pub fn evaluate_next_board(board: &[bool]) -> [bool; (SIZE * SIZE) as usize] {
    let mut next_board = [false; (SIZE * SIZE) as usize];

    for i in 0..SIZE * SIZE {
        let row = i / SIZE;
        let col = i % SIZE;

        next_board[i as usize] = will_cell_live(board, row, col);
    }

    next_board
}

pub fn will_cell_live(board: &[bool], row: u16, col: u16) -> bool {
    let neighbours = count_neighbours(board, row, col);

    if get_cell_state(board, row, col) {
        neighbours == 2 || neighbours == 3
    } else {
        neighbours == 3
    }
}

pub fn get_cell_state(board: &[bool], row: u16, col: u16) -> bool {
    if col >= SIZE || row >= SIZE {
        panic!("Out of bounds");
    }

    board[(row * SIZE + col) as usize]
}

pub fn count_neighbours(board: &[bool], row: u16, col: u16) -> u8 {
    let mut count = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }

            if row + i >= SIZE + 1 || col + j >= SIZE + 1 || row + i < 1 || col + j < 1 {
                continue;
            } else {
                if get_cell_state(board, row + i - 1, col + j - 1) {
                    count += 1;
                }
            }
        }
    }

    count
}

// set gosper's glider gun
fn set_gosper_glider_gun(board: &mut [bool; (SIZE * SIZE) as usize]) {
    let pulsar = "\
    ..*.*....*.*..
    .............
    *....*..*....*
    *....*..*....*
    *....*..*....*
    ..*.*....*.*..
    .............
    ..*.*....*.*..
    *....*..*....*
    *....*..*....*
    *....*..*....*
    .............
    ..*.*....*.*..
";

let penta_decathlon = "\
    ..........
    ......*...
    ****..****
    ......*...
    ..........
";

     let glider_gun = "\
        ........................#............
        ......................#.#............
        ............##......##............##..
        ...........#...#....##............##..
        ##........#.....#...##...............
        ##........#...#.##....#.#............
        ..........#.....#.......#............
        ...........#...#.....................
        ............##.......................
    ";
    
    let mut row = 0;
    let mut col = 0;
    for ch in pulsar.chars() {
        match ch {
            '.' => col += 1,
            '*' |
            '#' => {
                let index = row * SIZE + col;
                board[index as usize] = true;
                col += 1;
            },
            '\n' => {
                row += 1;
                col = 0;
            },
            _ => (),
        }
    }


}
