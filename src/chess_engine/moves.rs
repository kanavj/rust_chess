use super::board::*;
use super::piece::*;

#[derive(Copy, Clone, Debug)]
pub enum CastleSide {
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    EnPassant(EnPassantMove),
    Castles(CastlesMove),
    Normal(NormalMove),
    Promotion(PromotionMove),
}

#[derive(Copy, Clone, Debug)]
pub struct PromotionMove {
    pub from_position: (u8, u8),
    pub to_position: (u8, u8),
    pub new_piece: Piece,
}

#[derive(Copy, Clone, Debug)]
pub struct CastlesMove {
    pub color: Color,
    pub side: CastleSide,
}

#[derive(Copy, Clone, Debug)]
pub struct EnPassantMove {
    pub from_position: (u8, u8),
    pub to_position: (u8, u8),
    pub pawn_capture_position: (u8, u8),
}

#[derive(Copy, Clone, Debug)]
pub struct NormalMove {
    pub piece: Piece,
    pub from_position: (u8, u8),
    pub to_position: (u8, u8),
    pub capture: Option<Piece>,
}

impl Board {
    fn get_all_moves(&self, color: Color) -> Vec<Move> {
        let mut all_moves: Vec<Move> = Vec::new();

        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if let Some(piece) = self.board[i][j]
                    && piece.color == color
                {
                    let mut piece_moves = self.get_piece_moves((i as u8, j as u8));
                    all_moves.append(&mut piece_moves);
                }
            }
        }
        all_moves
    }

    pub fn get_piece_legal_moves(&self, from_position: (u8, u8)) -> Vec<Move> {
        let mut piece_legal_mvoes: Vec<Move> = Vec::new();
        let piece = match self.board[from_position.0 as usize][from_position.1 as usize] {
            Some(p) => p,
            None => return piece_legal_mvoes,
        };
        let candidate_moves: Vec<Move> = self.get_piece_moves(from_position);

        for mv in candidate_moves {
            let mut copyboard = self.clone();
            // Simulate the next move
            copyboard.make_move(mv);

            // If castling, place some kings to check if can castle
            match mv {
                Move::Castles(castles_mv) => {
                    let new_king = Piece {
                        color: piece.color,
                        piece_type: PieceType::King,
                        has_moved: false,
                    };
                    let row: usize = match castles_mv.color {
                        Color::White => 0,
                        Color::Black => 7,
                    };
                    match castles_mv.side {
                        CastleSide::King => {
                            copyboard.board[row][4] = Some(new_king);
                            copyboard.board[row][5] = Some(new_king);
                        }
                        CastleSide::Queen => {
                            copyboard.board[row][3] = Some(new_king);
                        }
                    }
                }
                _ => {}
            };

            // If we are in check after the move, it's not legal
            if copyboard.in_check(piece.color) {
                continue;
            }
            piece_legal_mvoes.push(mv);
        }
        piece_legal_mvoes
    }

    pub fn get_all_legal_moves(&self, color: Color) -> Vec<Move> {
        let mut all_legal_moves: Vec<Move> = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.color == color {
                        all_legal_moves.append(&mut self.get_piece_legal_moves((i as u8, j as u8)))
                    }
                }
            }
        }
        all_legal_moves
    }

    pub fn in_check(&self, color: Color) -> bool {
        for mv in self.get_all_moves(color.opposite()) {
            match mv {
                Move::Normal(normal_move) => {
                    if let Some(piece) = normal_move.capture {
                        if piece.piece_type == PieceType::King && piece.color == color {
                            return true;
                        }
                    }
                }
                _ => {}
            };
        }
        return false;
    }

    pub fn make_move(&mut self, mv: Move) {
        match mv {
            Move::Normal(normal_move) => self.make_normal_move(normal_move),
            Move::Castles(castles_move) => self.make_castles_move(castles_move),
            Move::EnPassant(ep_move) => self.make_enpassant_move(ep_move),
            Move::Promotion(pr_move) => self.make_promotion_move(pr_move),
        }
        self.last_move = mv;
    }

    fn make_normal_move(&mut self, mv: NormalMove) {
        if let Some(mut piece) =
            self.board[mv.from_position.0 as usize][mv.from_position.1 as usize]
        {
            self.board[mv.from_position.0 as usize][mv.from_position.1 as usize] = None;
            piece.has_moved = true;
            self.board[mv.to_position.0 as usize][mv.to_position.1 as usize] = Some(piece);
        }
    }

    fn make_castles_move(&mut self, mv: CastlesMove) {
        let king_starting_col: u8 = 4;
        let (king_ending_col, row, rook_starting_col, rook_ending_col): (u8, u8, u8, u8);
        match mv.side {
            CastleSide::King => {
                king_ending_col = 6;
                rook_starting_col = 7;
                rook_ending_col = 5;
            }
            CastleSide::Queen => {
                king_ending_col = 2;
                rook_starting_col = 0;
                rook_ending_col = 3;
            }
        };
        match mv.color {
            Color::Black => {
                row = 7;
            }
            Color::White => {
                row = 0;
            }
        };
        self.board[row as usize][king_starting_col as usize] = None;
        self.board[row as usize][king_ending_col as usize] = Some(Piece {
            piece_type: PieceType::King,
            color: mv.color,
            has_moved: true,
        });
        self.board[row as usize][rook_starting_col as usize] = None;
        self.board[row as usize][rook_ending_col as usize] = Some(Piece {
            piece_type: PieceType::Rook,
            color: mv.color,
            has_moved: true,
        });
    }

    fn make_promotion_move(&mut self, mv: PromotionMove) {
        self.board[mv.from_position.0 as usize][mv.from_position.1 as usize] = None;
        self.board[mv.to_position.0 as usize][mv.to_position.1 as usize] = Some(mv.new_piece);
    }

    fn make_enpassant_move(&mut self, mv: EnPassantMove) {
        if let Some(piece) = self.board[mv.from_position.0 as usize][mv.from_position.1 as usize] {
            self.board[mv.to_position.0 as usize][mv.to_position.1 as usize] = Some(piece);
        }
        self.board[mv.from_position.0 as usize][mv.from_position.1 as usize] = None;
        self.board[mv.pawn_capture_position.0 as usize][mv.pawn_capture_position.1 as usize] = None;
    }
}
