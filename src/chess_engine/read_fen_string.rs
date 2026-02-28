use super::game::{Color, Game};
use super::piece::{Piece, PieceType};
use std::io::{self, Error, ErrorKind};

const FEN_LINE_SEPARATOR: char = '/';
const ALLOWED_LOWERCASE_CHARS: [char; 6] = ['p', 'b', 'r', 'n', 'k', 'q'];

impl Game {
    pub fn from_fen_str(fen: &str) -> Result<Game, io::Error> {
        let mut game = Game::from_blank_board();
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
            let p = alg_to_piece(chr);
            let mut has_moved = false;
            if p.piece_type == PieceType::Pawn {
                if p.color == Color::White && row != 1 {
                    has_moved = true;
                }
                if p.color == Color::Black && row != 6 {
                    has_moved = true;
                }
            }
            let my_piece = Piece {
                color: p.color,
                piece_type: p.piece_type,
                has_moved: has_moved,
            };
            game.board[row as usize][col as usize] = Some(my_piece);
            col += 1;
        }
        game.next_legal_moves = game.get_all_legal_moves(true);
        return Ok(game);
    }

    pub fn to_fen_str(&self) -> String {
        let mut result = String::new();
        let mut blanks = 0;
        for i in (0..8).rev() {
            for j in 0..8 {
                let piece = self.board[i][j];
                match piece {
                    Some(p) => {
                        let piece_name = piece_to_alg(p);
                        if blanks > 0 {
                            result.push(char::from_digit(blanks, 10).unwrap());
                            blanks = 0;
                        }
                        result.push(piece_name);
                    }
                    None => {
                        blanks += 1;
                    }
                }
            }
            if blanks > 0 {
                result.push(char::from_digit(blanks, 10).unwrap());
                blanks = 0;
            }
            if i != 0 {
                result.push('/');
            }
        }
        result
    }
}

fn check_fen_str(fen: &str) -> Result<(), Error> {
    let mut line_length: u32 = 0;
    let mut total_lines = 0;

    for chr in fen.chars() {
        if chr.is_ascii_digit() {
            let digit = chr.to_digit(10).unwrap();
            if digit == 0 {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Can't put 0 as a number buddy",
                ));
            }
            if digit > 8 {
                return Err(Error::new(ErrorKind::Other, "Number can't be more than 8"));
            } else {
                line_length += digit;
                continue;
            }
        }
        if chr == FEN_LINE_SEPARATOR {
            if line_length != 8 {
                if line_length < 8 {
                    return Err(Error::new(ErrorKind::Other, "Line length too short buddy"));
                } else {
                    return Err(Error::new(ErrorKind::Other, "Line length too long buddy"));
                }
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

pub fn piece_to_alg(piece: Piece) -> char {
    let mut piece_name = match piece.piece_type {
        PieceType::Pawn => 'p',
        PieceType::Rook => 'r',
        PieceType::Knight => 'n',
        PieceType::Bishop => 'b',
        PieceType::King => 'k',
        PieceType::Queen => 'q',
    };
    if piece.color == Color::White {
        piece_name = piece_name.to_ascii_uppercase();
    }
    piece_name
}

pub fn alg_to_piece(name: char) -> Piece {
    let mut color = Color::Black;
    if name.is_uppercase() {
        color = Color::White;
    }
    let l_chr = name.to_ascii_lowercase();
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
    Piece {
        color,
        piece_type: p_type,
        has_moved: false,
    }
}
