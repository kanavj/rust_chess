use super::game::*;
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
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub new_piece: Piece,
}

#[derive(Copy, Clone, Debug)]
pub struct CastlesMove {
    pub color: Color,
    pub side: CastleSide,
}

#[derive(Copy, Clone, Debug)]
pub struct EnPassantMove {
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub pawn_capture_position: (usize, usize),
}

#[derive(Copy, Clone, Debug)]
pub struct NormalMove {
    pub piece: Piece,
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub capture: Option<Piece>,
}

impl Game {
    fn get_all_moves(&mut self, color: Color) -> Vec<Move> {
        let mut all_moves: Vec<Move> = Vec::new();

        for i in 0..self.board.len() {
            for j in 0..self.board[0].len() {
                if let Some(piece) = self.board[i][j]
                    && piece.color == color
                {
                    let mut piece_moves = self.get_piece_moves((i, j));
                    all_moves.append(&mut piece_moves);
                }
            }
        }
        all_moves
    }

    pub fn get_piece_legal_moves(&mut self, from_position: (usize, usize)) -> Vec<Move> {
        let mut piece_legal_moves: Vec<Move> = Vec::new();
        let piece = match self.board[from_position.0][from_position.1] {
            Some(p) => p,
            None => return piece_legal_moves,
        };
        let candidate_moves: Vec<Move> = self.get_piece_moves(from_position);

        for mv in candidate_moves {
            let mut copyboard = self.clone();
            // Simulate the next move
            copyboard.make_move(mv, false);

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

            // If we are in check after the move, it's not legal and add all the next moves to the dict
            if copyboard.in_check(piece.color) {
                continue;
            }
            piece_legal_moves.push(mv);
        }
        piece_legal_moves
    }

    pub fn get_all_legal_moves(&mut self) -> Vec<Move> {
        let color = self.next_player;
        let mut all_legal_moves: Vec<Move> = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.color == color {
                        all_legal_moves.append(&mut self.get_piece_legal_moves((i, j)))
                    }
                }
            }
        }
        all_legal_moves
    }

    pub fn print_all_legal_moves(&self) {
        for mv in &self.next_legal_moves {
            println!("{:?}", mv);
        }
    }

    pub fn in_check(&mut self, color: Color) -> bool {
        let opposite_moves = self.get_all_moves(color.opposite());
        for mv in &opposite_moves {
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

    pub fn make_move(&mut self, mv: Move, change_state: bool) {
        match mv {
            Move::Normal(normal_move) => self.make_normal_move(normal_move),
            Move::Castles(castles_move) => self.make_castles_move(castles_move),
            Move::EnPassant(ep_move) => self.make_enpassant_move(ep_move),
            Move::Promotion(pr_move) => self.make_promotion_move(pr_move),
        }
        // Add move to history
        self.move_history.push(mv);
        // Change color
        self.next_player = self.next_player.opposite();
        if change_state {
            // Calculate next moves
            self.next_legal_moves = self.get_all_legal_moves();
            // Check if the next player is in check
            let in_check = self.in_check(self.next_player);
            // Change board state
            if self.next_legal_moves.len() == 0 {
                if in_check {
                    self.state = GameState::Checkmate(self.next_player);
                } else {
                    self.state = GameState::Stalemate;
                }
            } else {
                if in_check {
                    self.state = GameState::InCheck(self.next_player);
                } else {
                    self.state = GameState::Normal;
                }
            }
        }
    }

    fn make_normal_move(&mut self, mv: NormalMove) {
        if let Some(mut piece) = self.board[mv.from_position.0][mv.from_position.1] {
            self.board[mv.from_position.0][mv.from_position.1] = None;
            piece.has_moved = true;
            self.board[mv.to_position.0][mv.to_position.1] = Some(piece);
        }
    }

    fn make_castles_move(&mut self, mv: CastlesMove) {
        let king_starting_col: usize = 4;
        let (king_ending_col, row, rook_starting_col, rook_ending_col): (
            usize,
            usize,
            usize,
            usize,
        );
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
        self.board[row][king_starting_col] = None;
        self.board[row][king_ending_col] = Some(Piece {
            piece_type: PieceType::King,
            color: mv.color,
            has_moved: true,
        });
        self.board[row][rook_starting_col] = None;
        self.board[row][rook_ending_col] = Some(Piece {
            piece_type: PieceType::Rook,
            color: mv.color,
            has_moved: true,
        });
    }

    fn make_promotion_move(&mut self, mv: PromotionMove) {
        self.board[mv.from_position.0][mv.from_position.1] = None;
        self.board[mv.to_position.0][mv.to_position.1] = Some(mv.new_piece);
    }

    fn make_enpassant_move(&mut self, mv: EnPassantMove) {
        if let Some(piece) = self.board[mv.from_position.0][mv.from_position.1] {
            self.board[mv.to_position.0][mv.to_position.1] = Some(piece);
        }
        self.board[mv.from_position.0][mv.from_position.1] = None;
        self.board[mv.pawn_capture_position.0][mv.pawn_capture_position.1] = None;
    }
}
