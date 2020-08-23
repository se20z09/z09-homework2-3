use std::io;

mod board;

use board::Board;


fn main() {
    let mut input = String::new();
    //let mut board = Board::new_random(3, 3);
    let mut test_board = vec![vec![true, false, true], vec![false, false, false], vec![true, true, false]];
    let mut board = Board::new_from_vector(test_board);
    println!("Welcome to the Game of Life! Press Enter to start another generation, or 'q and Enter to quit");
    
    while input.trim() != "q" {
        board.gen();
        io::stdin()
            .read_line(&mut input)
            .expect("Bad input");
        println!("Press Enter to start another generation or 'q' and Enter to quit");
    }

    println!("Finished!");
}
