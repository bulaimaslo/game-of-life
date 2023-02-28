pub(crate) mod shapes;

const MAX_SIZE: u16 = 100;

pub struct Board {
    size: u16,
    board: Vec<bool>,
}


impl Board {
    pub fn new(size: u16) -> Self {
        if size > MAX_SIZE {
            panic!("Board size too large");
        }
        Self {
            size,
            board: vec![false; (size * size) as usize],
        }
    }


    pub fn get_cell_state(&self, row: u16, col: u16) -> bool {
        if col >= self.size || row >= self.size {
            panic!("Out of bounds");
        }

        self.board[(row * self.size + col) as usize]
    }

    pub fn get_cell_state_index(&self, index: usize) -> bool {
        if index >= (self.size * self.size) as usize {
            panic!("Out of bounds");
        }
        self.board[index]
    }


    pub fn set_cell_state(&mut self, row: u16, col: u16, state: bool) {
        if col >= self.size || row >= self.size {
            panic!("Out of bounds");
        }

        self.board[(row * self.size + col) as usize] = state;
    }

    pub fn count_neighbours(&self, row: u16, col: u16) -> u8 {
        let mut count = 0;

        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                if row + i >= self.size + 1 || col + j >= self.size + 1 || row + i < 1 || col + j < 1 {
                    continue;
                } else {
                    if self.get_cell_state(row + i - 1, col + j - 1) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn will_cell_live(&self, row: u16, col: u16) -> bool {
        let neighbours = self.count_neighbours(row, col);

        if self.get_cell_state(row, col) {
            neighbours == 2 || neighbours == 3
        } else {
            neighbours == 3
        }
    }

    pub fn evaluate_next_board(&self) -> Board {
        let mut next_board = Board::new(self.size);

        for i in 0..self.size * self.size {
            let row = i / self.size;
            let col = i % self.size;

            next_board.board[i as usize] = self.will_cell_live(row, col);
        }

        next_board
    }

    pub fn set_shape(&mut self, shape: shapes::Shape) {
        let mut row = 0;

        let shape_str = shape.get_shape_string();

        for line in shape_str.lines() {
            let mut col = 0;

            for c in line.chars() {
                if c == '*' || c == '#' {
                    self.set_cell_state(row, col, true);
                }

                col += 1;
            }

            row += 1;
        }
    }
}