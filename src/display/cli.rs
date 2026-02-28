use crate::Display;
use crate::chess_engine::{game::Game, moves::*, read_fen_string::*};

use std::io::stdin;

pub struct CLIDisplayer;

impl CLIDisplayer {
    fn display_moves(&self, game: &Game) {
        let possible_moves = game.mvs_to_str();
        for (i, key) in possible_moves.keys().enumerate() {
            println!("{}. {}", i + 1, key);
        }
    }
}

impl Display for CLIDisplayer {
    fn display(&self, game: &Game) {
        let (n_rows, n_cols) = (game.board.len(), game.board[0].len());

        let (row_pad, col_pad) = (1, 2);

        print_horizontal_sep(n_cols, col_pad);

        for i in 0..n_rows {
            for _ in 0..row_pad {
                print_empty_row(n_cols, col_pad);
            }

            print!("|");
            for j in 0..n_cols {
                for _ in 0..col_pad {
                    print!(" ");
                }
                let p = match game.board[n_rows - i - 1][j] {
                    Some(piece) => piece_to_alg(piece),
                    None => ' ',
                };
                print!("{}", p);
                for _ in 0..col_pad {
                    print!(" ");
                }
                print!("|");
            }
            println!("");

            for _ in 0..row_pad {
                print_empty_row(n_cols, col_pad);
            }

            print_horizontal_sep(n_cols, col_pad);
        }

        println!("Possible moves:");
        self.display_moves(game);
    }

    fn display_message(&self, message: String) {
        println!("{}", message);
    }

    fn user_input(&self, game: &Game) -> Move {
        let possible_moves = game.mvs_to_str();
        let mut user_input = String::new();

        stdin()
            .read_line(&mut user_input)
            .expect("Enter a valid string please");
        user_input = user_input.trim().to_string();

        while !(possible_moves.contains_key(&user_input)) {
            println!("Please enter one of the valid moves:");
            self.display_moves(game);

            user_input.clear();
            stdin()
                .read_line(&mut user_input)
                .expect("Enter a valid string please");
            user_input = user_input.trim().to_string();
        }
        possible_moves[&user_input]
    }
}

fn print_horizontal_sep(n_cols: usize, col_pad: usize) {
    for _ in 0..(n_cols * (2 + 2 * col_pad) + 1) {
        print!("-");
    }
    println!("");
}

fn print_empty_row(n_cols: usize, col_pad: usize) {
    print!("|");
    for _ in 0..n_cols {
        for _ in 0..(2 * col_pad + 1) {
            print!(" ");
        }
        print!("|");
    }
    println!("");
}
