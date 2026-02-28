use std::collections::HashMap;

use super::game::*;
use super::moves::*;
use super::piece::*;

impl Game {
    pub fn mvs_to_str(&self) -> HashMap<String, Move> {
        let mut movemap: HashMap<String, Vec<Move>> = HashMap::new();
        let mut output_map = HashMap::new();

        for mv in &self.next_legal_moves {
            let mvstr = self.mv_to_str(*mv);
            movemap.entry(mvstr).or_insert(Vec::new()).push(*mv);
        }

        for (key, item) in movemap {
            if item.len() == 1 {
                output_map.insert(
                    format!("{}{}", key, game_state_to_str(item[0].get_state())),
                    item[0],
                );
            } else {
                let deduplicated = deduplicate_moves(&key, item);
                for (str, mv) in deduplicated {
                    output_map.insert(format!("{}{}", str, game_state_to_str(mv.get_state())), mv);
                }
            }
        }
        output_map
    }

    fn mv_to_str(&self, mv: Move) -> String {
        match mv {
            Move::Normal(mv) => {
                let is_capture = mv.capture.is_some();
                let to_str = board_position_to_notation(mv.to_position.0, mv.to_position.1);
                let piece_char: char = match mv.piece.piece_type {
                    PieceType::Pawn => {
                        // If capturing, we need the file
                        if is_capture {
                            board_position_to_notation(mv.from_position.0, mv.from_position.1)
                                .chars()
                                .next()
                                .expect("what happened lol")
                        } else {
                            // Will be trimmed off
                            ' '
                        }
                    }
                    PieceType::Bishop => 'B',
                    PieceType::Queen => 'Q',
                    PieceType::Knight => 'N',
                    PieceType::Rook => 'R',
                    PieceType::King => 'K',
                };
                return format!(
                    "{}{}{}",
                    piece_char,
                    if is_capture { "x" } else { "" },
                    to_str,
                )
                .trim()
                .to_string();
            }
            Move::Castles(mv) => {
                match mv.side {
                    CastleSide::King => {
                        return format!("O-O");
                    }
                    CastleSide::Queen => {
                        return format!("O-O-O");
                    }
                };
            }
            Move::Promotion(mv) => {
                let normal_mv_str = self.mv_to_str(Move::Normal(NormalMove {
                    piece: mv.piece,
                    from_position: mv.from_position,
                    to_position: mv.to_position,
                    capture: mv.capture,
                    game_state: GameState::Normal,
                }));
                let newpiece_char = match mv.new_piece.piece_type {
                    PieceType::Bishop => 'B',
                    PieceType::Knight => 'N',
                    PieceType::Queen => 'Q',
                    PieceType::Rook => 'R',
                    _ => {
                        panic!("Can't make this wtf")
                    }
                };
                return format!("{}={}", normal_mv_str, newpiece_char);
            }
            Move::EnPassant(mv) => {
                let from_file = board_position_to_notation(mv.from_position.0, mv.from_position.1)
                    .chars()
                    .next()
                    .expect("wtf lol");
                let to_position = board_position_to_notation(mv.to_position.0, mv.to_position.1);
                return format!("{}x{}", from_file, to_position);
            }
        };
    }
}

fn deduplicate_moves(movestr: &str, mvs: Vec<Move>) -> Vec<(String, Move)> {
    let mut output = Vec::new();

    let normal_moves: Vec<&NormalMove> = mvs
        .iter()
        .filter_map(|mv| match mv {
            Move::Normal(nmv) => Some(nmv),
            _ => panic!("wtf happened?"),
        })
        .collect();

    for &mv in &normal_moves {
        let mv_notation = board_position_to_notation(mv.from_position.0, mv.from_position.1);

        let has_same_col = normal_moves
            .iter()
            .any(|other| **other != *mv && other.from_position.1 == mv.from_position.1);

        let has_same_row = normal_moves
            .iter()
            .any(|other| **other != *mv && other.from_position.0 == mv.from_position.0);

        let condition = if !has_same_col {
            mv_notation.chars().nth(0).unwrap().to_string()
        } else if !has_same_row {
            mv_notation.chars().nth(1).unwrap().to_string()
        } else {
            mv_notation
        };

        let newstr = format!("{}{}{}", &movestr[0..1], condition, &movestr[1..]);
        output.push((newstr, Move::Normal(*mv)));
    }

    output
}

fn game_state_to_str(state: GameState) -> String {
    match state {
        GameState::InCheck(_) => {
            return "+".to_string();
        }
        GameState::Normal => {
            return "".to_string();
        }
        GameState::Checkmate(_) => {
            return "#".to_string();
        }
        GameState::Stalemate => {
            return "".to_string();
        }
    }
}
