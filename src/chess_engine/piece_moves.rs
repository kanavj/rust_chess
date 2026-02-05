use super::game::*;
use super::moves::*;
use crate::chess_engine::piece::{Piece, PieceType};

impl Game {
    pub fn get_piece_moves(&self, from_position: (usize, usize)) -> Vec<Move> {
        // Check if piece is present
        match self.board[from_position.0][from_position.1] {
            Some(piece) => match piece.piece_type {
                PieceType::Knight => self.knight_legal_moves(from_position, piece),
                PieceType::Bishop => self.bishop_legal_moves(from_position, piece),
                PieceType::Rook => self.rook_legal_moves(from_position, piece),
                PieceType::Queen => self.queen_legalmoves(from_position, piece),
                PieceType::King => self.king_legal_moves(from_position, piece),
                PieceType::Pawn => self.pawn_legal_moves(from_position, piece),
            },
            None => Vec::new(),
        }
    }

    // Knight ez
    pub fn knight_legal_moves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let moves = [
            (-2, -1),
            (-1, -2),
            (2, -1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, 1),
            (1, 2),
        ];
        let possible_moves: Vec<Move> = moves
            .iter()
            .filter_map(|(dx, dy)| {
                let new_row = from_position.0 as i32 + dx;
                let new_col = from_position.1 as i32 + dy;
                if new_col < 0 || new_row < 0 {
                    return None;
                }
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                let to_position = (new_row, new_col);
                if valid_position(to_position) {
                    let capture = match self.board[new_row][new_col] {
                        Some(other_piece) => {
                            if piece.color == other_piece.color {
                                return None;
                            } else {
                                Some(other_piece)
                            }
                        }
                        None => None,
                    };
                    let my_move = Move::Normal(NormalMove {
                        piece: piece,
                        from_position: from_position,
                        to_position,
                        capture: capture,
                    });
                    Some(my_move)
                } else {
                    None
                }
            })
            .collect();
        possible_moves
    }

    // Helper function to go along a direction until you reach the end of the board
    fn sliding_moves_in_dir(
        &self,
        from_position: (usize, usize),
        dir: (i32, i32),
        max_dist: Option<i32>,
    ) -> impl Iterator<Item = ((usize, usize), Option<Piece>)> {
        let max_dist = match max_dist {
            Some(number) => number,
            None => 7,
        };
        (1..max_dist + 1).filter_map(move |i| {
            let new_row = from_position.0 as i32 + dir.0 * i;
            let new_col = from_position.1 as i32 + dir.1 * i;
            let to_position = (new_row as usize, new_col as usize);

            if valid_position(to_position) {
                match self.board[new_row as usize][new_col as usize] {
                    Some(other_piece) => Some((to_position, Some(other_piece))),
                    None => Some((to_position, None)),
                }
            } else {
                None
            }
        })
    }

    pub fn bishop_legal_moves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let mut possible_moves: Vec<Move> = Vec::new();
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for dir in directions {
            for (to_position, capture) in self.sliding_moves_in_dir(from_position, dir, None) {
                let my_move = NormalMove {
                    piece: piece,
                    from_position: from_position,
                    to_position,
                    capture: capture,
                };
                // Check if can capture something on the way
                if let Some(other_piece) = capture {
                    if piece.color != other_piece.color {
                        possible_moves.push(Move::Normal(my_move));
                        break;
                    } else {
                        break;
                    }
                }
                possible_moves.push(Move::Normal(my_move));
            }
        }
        possible_moves
    }

    pub fn rook_legal_moves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for dir in directions {
            for (to_position, capture) in self.sliding_moves_in_dir(from_position, dir, None) {
                let my_move = NormalMove {
                    piece: piece,
                    from_position: from_position,
                    to_position,
                    capture: capture,
                };
                // Check if can capture something on the way
                if let Some(other_piece) = capture {
                    if piece.color != other_piece.color {
                        possible_moves.push(Move::Normal(my_move));
                        break;
                    } else {
                        break;
                    }
                }
                possible_moves.push(Move::Normal(my_move));
            }
        }
        possible_moves
    }

    pub fn queen_legalmoves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let mut b_moves = self.bishop_legal_moves(from_position, piece);
        b_moves.append(&mut self.rook_legal_moves(from_position, piece));
        b_moves
    }

    pub fn king_legal_moves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        let directions = [
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for dir in directions {
            for (to_position, capture) in self.sliding_moves_in_dir(from_position, dir, Some(1)) {
                let my_move = NormalMove {
                    piece: piece,
                    from_position: from_position,
                    to_position,
                    capture: capture,
                };
                // Check if can capture something on the way
                if let Some(other_piece) = capture {
                    if piece.color != other_piece.color {
                        possible_moves.push(Move::Normal(my_move));
                        break;
                    } else {
                        break;
                    }
                }
                possible_moves.push(Move::Normal(my_move));
            }
        }

        // Add castles moves
        let mut castles_moves = self.king_castles_moves(piece);
        possible_moves.append(&mut castles_moves);
        possible_moves
    }

    pub fn king_castles_moves(&self, piece: Piece) -> Vec<Move> {
        let mut castle_moves = Vec::new();
        if piece.has_moved {
            return castle_moves;
        }
        let row = match piece.color {
            Color::White => 0,
            Color::Black => 7,
        };
        // Check kingside castle
        if self.board[row][5] == None && self.board[row][6] == None {
            if let Some(rook_piece) = self.board[row][7] {
                if !rook_piece.has_moved
                    && rook_piece.piece_type == PieceType::Rook
                    && rook_piece.color == piece.color
                {
                    castle_moves.push(Move::Castles(CastlesMove {
                        color: piece.color,
                        side: CastleSide::King,
                    }));
                }
            }
        }

        // Check queenside castle
        if self.board[row][1] == None && self.board[row][2] == None && self.board[row][3] == None {
            if let Some(rook_piece) = self.board[row][0] {
                if !rook_piece.has_moved
                    && rook_piece.piece_type == PieceType::Rook
                    && rook_piece.color == piece.color
                {
                    castle_moves.push(Move::Castles(CastlesMove {
                        color: piece.color,
                        side: CastleSide::Queen,
                    }));
                }
            }
        }
        castle_moves
    }

    pub fn pawn_legal_moves(&self, from_position: (usize, usize), piece: Piece) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        let (vertical_dir, capture_directions) = match piece.color {
            Color::White => ((1, 0), [(1, 1), (1, -1)]),
            Color::Black => ((-1, 0), [(-1, 1), (-1, -1)]),
        };

        // Don't capture here
        let distance = if piece.has_moved { 1 } else { 2 };
        for (to_position, capture) in
            self.sliding_moves_in_dir(from_position, vertical_dir, Some(distance))
        {
            if capture.is_some() {
                break;
            }
            add_pawn_move(
                &mut possible_moves,
                from_position,
                to_position,
                piece,
                capture,
            );
        }

        // Only capture here
        for dir in capture_directions {
            for (to_position, capture) in self.sliding_moves_in_dir(from_position, dir, Some(1)) {
                if let Some(other_piece) = capture {
                    if other_piece.color != piece.color {
                        add_pawn_move(
                            &mut possible_moves,
                            from_position,
                            to_position,
                            piece,
                            capture,
                        );
                    }
                }
            }
        }

        // En Passant moves
        if let Some(last_move) = self.move_history.last() {
            match *last_move {
                Move::Normal(mv) => {
                    let dist_moved = (mv.from_position.0 as i32 - mv.to_position.0 as i32).abs();
                    if mv.piece.has_moved == false
                        && dist_moved == 2
                        && mv.piece.piece_type == PieceType::Pawn
                    {
                        if from_position.0 == mv.to_position.0 {
                            let x_diff = from_position.1 as i32 - mv.to_position.1 as i32;
                            if x_diff.abs() == 1 {
                                possible_moves.push(Move::EnPassant(EnPassantMove {
                                    from_position: from_position,
                                    to_position: (
                                        (from_position.0 as i32 + vertical_dir.0) as usize,
                                        (from_position.1 as i32 - x_diff) as usize,
                                    ),
                                    pawn_capture_position: (mv.to_position),
                                }))
                            }
                        }
                    }
                }
                _ => {}
            };
        }

        possible_moves
    }
}

fn add_pawn_move(
    possible_moves: &mut Vec<Move>,
    from_position: (usize, usize),
    to_position: (usize, usize),
    piece: Piece,
    capture: Option<Piece>,
) {
    if check_pawn_promotion(to_position, piece) {
        possible_moves.extend_from_slice(&get_promotion_moves(
            from_position,
            to_position,
            piece,
            capture,
        ));
    } else {
        let my_move = NormalMove {
            piece: piece,
            from_position: from_position,
            to_position,
            capture: capture,
        };
        possible_moves.push(Move::Normal(my_move));
    }
}

fn check_pawn_promotion(to_position: (usize, usize), piece: Piece) -> bool {
    assert_eq!(piece.piece_type, PieceType::Pawn);
    (piece.color == Color::White && to_position.0 == 7)
        || (piece.color == Color::Black && to_position.0 == 0)
}

fn get_promotion_moves(
    from_position: (usize, usize),
    to_position: (usize, usize),
    piece: Piece,
    capture: Option<Piece>,
) -> [Move; 4] {
    return [
        Move::Promotion(PromotionMove {
            piece: piece,
            from_position: from_position,
            to_position: to_position,
            new_piece: Piece {
                color: piece.color,
                piece_type: PieceType::Bishop,
                has_moved: true,
            },
            capture: capture,
        }),
        Move::Promotion(PromotionMove {
            piece: piece,
            from_position: from_position,
            to_position: to_position,
            new_piece: Piece {
                color: piece.color,
                piece_type: PieceType::Knight,
                has_moved: true,
            },
            capture: capture,
        }),
        Move::Promotion(PromotionMove {
            piece: piece,
            from_position: from_position,
            to_position: to_position,
            new_piece: Piece {
                color: piece.color,
                piece_type: PieceType::Rook,
                has_moved: true,
            },
            capture: capture,
        }),
        Move::Promotion(PromotionMove {
            piece: piece,
            from_position: from_position,
            to_position: to_position,
            new_piece: Piece {
                color: piece.color,
                piece_type: PieceType::Queen,
                has_moved: true,
            },
            capture: capture,
        }),
    ];
}
