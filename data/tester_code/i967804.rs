use std::io;
use rand::Rng;
use std::convert::TryInto;

pub struct Board {
    pub row: usize,
    pub col: usize,
    pub board: Vec<Vec<bool>>
}

impl Board {
    // Return a randomized board of size row x col
    pub fn new_random(row: usize, col: usize) -> Board {
        let mut board = vec![vec![false; col]; row];
        let mut rng = rand::thread_rng();
        for i in 0..row{
            for j in 0..col{
                board[i][j] = rng.gen::<bool>();
            }
        }
        Board { row: row, col: col, board: board }
    }

    // Return a board specified by the user
    pub fn new_from_vector(user_board: Vec<Vec<bool>>) -> Board {
        Board { row: user_board.len(), col: user_board[0].len(), board: user_board }
    }

    fn render_board(&self) {
        for row in self.board.iter() {
            for cell in row.iter() {
                let sym = match *cell {
                    true => "o",
                    false => ".",
                };
                print!("{} ", sym);
            }
            println!("");
        }
        println!("");
    }

    fn count_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut neighbors = 0;
        // Check the immediate neighbors, using y for row index and x for cell index
        for x in -1..2 {
            for y in -1..2 {
                // 0,0 is the cell being queried
                if y == 0 && x == 0 {
                    continue;
                }
                // Calculate the index of the neighbor to check
                let x_ind: i32 = row + x;
                let y_ind: i32 = col + y;
                // Check the bounds, continue if index is out of bounds
                if x_ind < 0 || x_ind >= self.row.try_into().unwrap() {
                    continue;
                }

                if y_ind < 0 || y_ind >= self.col.try_into().unwrap() {
                    continue;
                }

                if self.board[x_ind as usize][y_ind as usize] {
                    neighbors += 1;
                }
            }
        }
        #[cfg(debug_assertions)]
        println!("Cell ({}, {}) has {} alive neighbors", row, col, neighbors);
        neighbors
    }

    pub fn gen(&mut self) -> bool {
        self.render_board();
        let mut new_board_vec = vec![vec![false; self.col]; self.row];

        for (r, row) in self.board.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                let neighbors = self.count_neighbors(c as i32, r as i32);
                let status = match (*cell, neighbors) {
                    (_, 3) => true,
                    (true, 2) => true,
                    (_, _) => false,
                };
                new_board_vec[r][c] = status;
            }
        }
        self.board = new_board_vec;
        true
    }
}


fn main() {
    let mut input = String::new();
    let mut board = Board::new_random(3, 3);
    // let test_board = vec![vec![true, false, true], vec![false, false, false], vec![true, true, false]];
    // let mut board = Board::new_from_vector(test_board);
    println!("Welcome to the Game of Life! Press Enter to start another generation, or 'q and Enter to quit");
    
    while input != "q" && input != "q\n" {
        board.gen();
        io::stdin()
            .read_line(&mut input)
            .expect("Bad input");
        println!("Press Enter to start another generation or 'q' and Enter to quit");
    }

    println!("Finished!");
}