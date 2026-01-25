pub mod chess_engine;
use crate::chess_engine::{board::*, moves::*, piece::*, read_fen_string::check_fen_str};

fn main() {
    let mut my_board = Board::new_standard_board();

    // let my_fen = "1r6/8/8/8/8/8/8/R3K2R";
    // println!("{:?}", check_fen_str(my_fen));

    // match my_board.from_fen_str(my_fen) {
    //     Ok(()) => {
    //         println!("{:?}", my_board.board);
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // };
    
    for row in my_board.board{
        println!("{:?}", row);
    }
    let all_moves = my_board.get_piece_legal_moves((0, 1));
    for mv in all_moves {
        println!("{:?}", mv);
    }
}
