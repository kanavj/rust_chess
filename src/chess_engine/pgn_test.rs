use super::{game, pgn_read};
use rayon::prelude::*;
use std::io::Error;

pub fn pgn_test(filepath: &str, games_to_read: usize) -> Result<(), Error> {
    let db_games = pgn_read::read_games(filepath, games_to_read)?;
    db_games.into_par_iter().enumerate().try_for_each(|(i, (db_game,_result))|{
            let mut my_game = game::Game::from_standard_board();

            for db_move in db_game {
                let moves_map = my_game.mvs_to_str();

                if let Some(&mv) = moves_map.get(&db_move) {
                    my_game.make_move(mv);
                    continue;
                }
                if let Some(&_mv) = moves_map.get(&db_move.replace("+", "#")){
                    continue;
                }
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!(
                        "game {}\n fen string: {} \nCould not locate move {} \n (legal moves: {:?})",
                        i,
                        my_game.to_fen_str(),
                        db_move,
                        moves_map.keys().collect::<Vec<&String>>(),
                        // my_game.move_history,
                    ),
                ));
            }
            println!("Finished game {}", i + 1);
            Ok(())
        }
    )
}
