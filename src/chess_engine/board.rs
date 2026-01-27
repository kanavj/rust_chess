use super::moves::*;
use crate::chess_engine::piece::{Piece, PieceType};
#[derive(Clone, Debug)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
    pub next_player: Color,
    pub last_move: Move,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub fn board_position_to_notation(row: usize, col: usize) -> String {
    if row > 8 || col > 8 {
        panic!("Row: {}, Height: {}. Cannot be greater than 8!", row, col);
    }
    let firstchar: char = ('A' as u8 + col as u8) as char;
    let secondchar: char = ('1' as u8 + row as u8) as char;
    return format!("{}{}", firstchar, secondchar);
}

pub fn valid_position(position: (usize, usize)) -> bool {
    if position.0 >= 8 || position.1 >= 8 {
        return false;
    }
    return true;
}

impl Board {
    pub fn new_blank_board() -> Board {
        let new_board: [[Option<Piece>; 8]; 8] = [[None; 8]; 8];

        // Making just a weird first move for no reason lule
        let sample_last_move = Move::Normal(NormalMove {
            piece: Piece {
                color: Color::Black,
                piece_type: PieceType::Queen,
                has_moved: false,
            },
            from_position: (2, 3),
            to_position: (2, 3),
            capture: None,
        });
        return Board {
            board: new_board,
            next_player: Color::White,
            last_move: sample_last_move,
        };
    }

    pub fn clear(&mut self) {
        for i in 0..8 {
            for j in 0..8 {
                self.board[i][j] = None;
            }
        }
    }

    pub fn new_standard_board() -> Board {
        let mut new_board = Self::new_blank_board();

        // Place white pieces first then mirror board

        // Pawns
        for i in 0..8 {
            let white_pawn = Piece {
                color: Color::White,
                piece_type: PieceType::Pawn,
                has_moved: false,
            };
            new_board.board[1][i] = Some(white_pawn);
        }

        // Rooks
        new_board.board[0][0] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: false,
        });
        new_board.board[0][7] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Rook,
            has_moved: false,
        });

        // Knights
        new_board.board[0][1] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: false,
        });
        new_board.board[0][6] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Knight,
            has_moved: false,
        });

        // Bishops
        new_board.board[0][2] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: false,
        });
        new_board.board[0][5] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Bishop,
            has_moved: false,
        });

        // Queen
        new_board.board[0][3] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::Queen,
            has_moved: false,
        });

        // King
        new_board.board[0][4] = Some(Piece {
            color: Color::White,
            piece_type: PieceType::King,
            has_moved: false,
        });

        // Mirror board
        for i in 0..2 {
            for j in 0..8 {
                let mut black_piece = match new_board.board[i][j] {
                    Some(piece) => piece,
                    None => continue,
                };
                black_piece.color = Color::Black;
                new_board.board[7 - i][j] = Some(black_piece);
            }
        }

        return new_board;
    }
}
