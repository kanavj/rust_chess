use super::board::{Board, Color};
use super::piece::{Piece, PieceType};
use std::io::{self, Error, ErrorKind};

const FEN_LINE_SEPARATOR: char = '/';
const ALLOWED_LOWERCASE_CHARS: [char; 6] = ['p', 'b', 'r', 'n', 'k', 'q'];

impl Board {
    pub fn from_fen_str(&mut self, fen: &str) -> Result<(), io::Error> {
        self.clear();
        check_fen_str(fen)?;

        let mut row = 7;
        let mut col = 0;

        for chr in fen.chars() {
            //Check if skipping
            if chr.is_ascii_digit() {
                let digit = chr.to_digit(10).unwrap();
                col += digit;
                continue;
            }
            // Check if next line
            if chr == '/' {
                row -= 1;
                col = 0;
                continue;
            }

            // Make piece
            let mut color = Color::Black;
            if chr.is_uppercase() {
                color = Color::White;
            }
            let l_chr = chr.to_ascii_lowercase();
            let p_type = match l_chr {
                'p' => PieceType::Pawn,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'k' => PieceType::King,
                'q' => PieceType::Queen,
                _ => {
                    panic!("How tf did this happen? What is that piece?")
                }
            };
            let mut has_moved = true;
            if p_type == PieceType::Pawn {
                if color == Color::White && row == 1 {
                    has_moved = false;
                }
                if color == Color::Black && row == 6 {
                    has_moved = false;
                }
            }
            let my_piece = Piece {
                color: color,
                piece_type: p_type,
                has_moved: has_moved,
            };
            self.board[row as usize][col as usize] = Some(my_piece);
            col += 1;
        }
        return Ok(());
    }
}

pub fn check_fen_str(fen: &str) -> Result<(), Error> {
    let mut line_length: u32 = 0;
    let mut total_lines = 0;

    for chr in fen.chars() {
        if chr.is_ascii_digit() {
            let digit = chr.to_digit(10).unwrap();
            if digit > 8 {
                return Err(Error::new(ErrorKind::Other, "Number can't be more than 8"));
            } else {
                line_length += digit;
                continue;
            }
        }
        if chr == FEN_LINE_SEPARATOR {
            if line_length != 8 {
                return Err(Error::new(ErrorKind::Other, "Line length too long buddy"));
            } else {
                line_length = 0;
                total_lines += 1;
                continue;
            }
        }
        if !ALLOWED_LOWERCASE_CHARS.contains(&chr.to_ascii_lowercase()) {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Char: {} cannot be in the string. The character(in lowercase) has to be one of {:?}",
                    chr, ALLOWED_LOWERCASE_CHARS,
                ),
            ));
        }
        line_length += 1;
    }

    if total_lines != 7 || line_length != 8 {
        return Err(Error::new(ErrorKind::Other, "Too few lines here buddy"));
    }
    return Ok(());
}
