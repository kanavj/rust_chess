pub mod chess_engine;
use crate::chess_engine::game::*;

fn main() {
    let my_fen = "8/7k/8/2R5/8/2R2R2/8/1K6";
    let mut game = Game::from_fen_str(my_fen).unwrap();
    // let mut game = Game::from_standard_board();
    game.next_player = Color::White;
    game.next_legal_moves = game.get_all_legal_moves();

    let mves = game.get_all_legal_moves();

    for (mv, str) in game.mvs_to_str(mves) {
        println!(
            "{}",
            // mv,
            str,
        );
    }

    // game.make_move(game.next_legal_moves[7], true);
    // println!("{:?}", game.state);
    // game.print_all_legal_moves();
}
