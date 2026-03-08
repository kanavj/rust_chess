use crate::Display;
use crate::game::{Game, GameState};

pub struct GameController<D: Display> {
    game: Game,
    displayer: D,
}

impl<D: Display> GameController<D> {
    pub fn new(game: Game, display: D) -> Self {
        GameController {
            game,
            displayer: display,
        }
    }

    pub async fn run(&mut self) {
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

            self.displayer.display(&self.game).await;

            let user_move = self.displayer.user_input(&self.game);
            if let Some(mv) = user_move {
                self.game.make_move(mv);
            }
        }
    }
}
