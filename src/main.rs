pub mod chess_engine;
use crate::chess_engine::game::*;

fn main() {
    let my_fen = "8/8/8/6q1/5k2/7K/8/8";
    let mut game = Game::from_fen_str(my_fen).unwrap();
    game.next_player = Color::Black;
    game.next_legal_moves = game.get_all_legal_moves();

    game.print_all_legal_moves();
    game.make_move(game.next_legal_moves[10], true);
    println!("{:?}", game.state);
    game.print_all_legal_moves();
}
