use crate::game::Game;
use crate::moves::Move;
pub mod cli;

pub trait Display {
    fn display(&self, game: &Game);
    fn user_input(&self, game: &Game) -> Move;
    fn display_message(&self, message: String);
}
