use crate::Display;
use crate::game::{Game, GameState};

pub struct GameController<'a, D: Display> {
    game: Game,
    displayer: &'a D,
}

impl<'a, D: Display> GameController<'a, D> {
    pub fn new(game: Game, display: &'a D) -> Self {
        GameController {
            game,
            displayer: display,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.game.state {
                GameState::Checkmate(col) => {
                    self.displayer.display_message(format!("{} loses!", col));
                    break;
                }
                GameState::Stalemate => {
                    self.displayer.display_message("Stalemate!".to_string());
                    break;
                }
                GameState::InCheck(col) => {
                    self.displayer.display_message(format!("{} in check!", col));
                }
                _ => {}
            }

            self.displayer.display(&self.game);

            let user_move = self.displayer.user_input(&self.game);
            self.game.make_move(user_move);
        }
    }
}
