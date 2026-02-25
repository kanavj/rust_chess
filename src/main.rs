pub mod chess_engine;
use crate::chess_engine::game::*;
use crate::chess_engine::pgn_test::*;

fn main() {
    let my_fen = "1q2r3/pp1n3k/1np3R1/7p/P6P/1QNBb3/1P3r2/1K1R4";
    let mut game = Game::from_fen_str(my_fen).unwrap();

    game.next_player = Color::White;
    game.next_legal_moves = game.get_all_legal_moves(true);
    let mvs = game.mvs_to_str();
    
    for mv in mvs {
        println!("{:?} \t  {}", mv.1, mv.0);
    }

    // pgn_test("Capablanca.pgn", 0).unwrap();
}
