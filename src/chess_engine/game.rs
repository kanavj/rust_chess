use core::fmt;

use super::moves::*;
use super::piece::{Piece, PieceType};

pub type BoardType = [[Option<Piece>; 8]; 8];

#[derive(Clone, Debug)]
pub struct Game {
    pub board: BoardType,
    pub next_player: Color,
    pub move_history: Vec<Move>,
    pub state: GameState,
    pub next_legal_moves: Vec<Move>,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum GameState {
    Normal,
    InCheck(Color),
    Checkmate(Color),
    Stalemate,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::Black => "Black",
            Color::White => "White",
        };
        write!(f, "{}", name)
    }
}

pub fn board_position_to_notation(row: usize, col: usize) -> String {
    if row > 8 || col > 8 {
        panic!("Row: {}, Height: {}. Cannot be greater than 8!", row, col);
    }
    let firstchar: char = ('a' as u8 + col as u8) as char;
    let secondchar: char = ('1' as u8 + row as u8) as char;
    return format!("{}{}", firstchar, secondchar);
}

pub fn valid_position(position: (usize, usize)) -> bool {
    if position.0 >= 8 || position.1 >= 8 {
        return false;
    }
    return true;
}

impl Game {
    pub fn from_blank_board() -> Game {
        let blank_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        let mut new_game = Game {
            board: blank_board,
            next_player: Color::White,
            move_history: Vec::new(),
            state: GameState::Normal,
            next_legal_moves: Vec::new(),
        };
        new_game.next_legal_moves = new_game.get_all_legal_moves(true);
        return new_game;
    }

    pub fn clear(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                self.board[i][j] = None;
            }
        }
    }

    pub fn from_standard_board() -> Game {
        let mut new_game = Self::from_blank_board();

        // Place white pieces first then mirror board

        // Pawns
        for i in 0..8 {
            let white_pawn = Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                has_moved: false,
            };
            new_game.board[1][i] = Some(white_pawn);
        }

        // Rooks
        new_game.board[0][0] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: false,
        });
        new_game.board[0][7] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: false,
        });

        // Knights
        new_game.board[0][1] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: false,
        });
        new_game.board[0][6] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: false,
        });

        // Bishops
        new_game.board[0][2] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: false,
        });
        new_game.board[0][5] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: false,
        });

        // Queen
        new_game.board[0][3] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
            has_moved: false,
        });

        // King
        new_game.board[0][4] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::King,
            has_moved: false,
        });

        // Mirror board
        for i in 0..2 {
            for j in 0..8 {
                let mut black_piece = match new_game.board[i][j] {
                    Some(piece) => piece,
                    None => continue,
                };
                black_piece.color = Color::Black;
                new_game.board[7 - i][j] = Some(black_piece);
            }
        }

        new_game.next_legal_moves = (&new_game).get_all_legal_moves(true);

        return new_game;
    }

    pub fn print_board(&self) {
        for row in &self.board {
            println!("{:?}", row);
        }
    }
}
