use super::game::*;
use super::piece::*;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum CastleSide {
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub enum Move {
    EnPassant(EnPassantMove),
    Castles(CastlesMove),
    Normal(NormalMove),
    Promotion(PromotionMove),
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct PromotionMove {
    pub piece: Piece,
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub capture: Option<Piece>,
    pub new_piece: Piece,
    pub game_state: GameState,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct CastlesMove {
    pub color: Color,
    pub side: CastleSide,
    pub game_state: GameState,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct EnPassantMove {
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub pawn_capture_position: (usize, usize),
    pub game_state: GameState,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct NormalMove {
    pub piece: Piece,
    pub from_position: (usize, usize),
    pub to_position: (usize, usize),
    pub capture: Option<Piece>,
    pub game_state: GameState,
}

impl Move {
    fn set_state(&mut self, state: GameState) {
        match self {
            Move::Normal(mv) => mv.game_state = state,
            Move::Castles(mv) => mv.game_state = state,
            Move::EnPassant(mv) => mv.game_state = state,
            Move::Promotion(mv) => mv.game_state = state,
        };
    }

    pub fn get_state(&self) -> GameState {
        match self {
            Move::Normal(mv) => mv.game_state,
            Move::Castles(mv) => mv.game_state,
            Move::EnPassant(mv) => mv.game_state,
            Move::Promotion(mv) => mv.game_state,
        }
    }
}

impl Game {
    fn get_all_moves(&self, color: Color) -> Vec<Move> {
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

    fn get_piece_legal_moves(
        &self,
        from_position: (usize, usize),
        check_next: bool,
    ) -> Vec<Move> {
        let mut piece_legal_moves: Vec<Move> = Vec::new();
        let piece = match self.board[from_position.0][from_position.1] {
            Some(p) => p,
            None => return piece_legal_moves,
        };
        let mut candidate_moves: Vec<Move> = self.get_piece_moves(from_position);

        for mv in &mut candidate_moves {
            let mut copyboard = self.clone();
            // Simulate the next move
            copyboard.make_move_helper(*mv, false);

            // If castling, place some kings to check if can castle
            match mv {
                Move::Castles(castles_mv) => {
                    let new_king = Piece {
                        color: piece.color,
                        piece_type: PieceType::King,
                        has_moved: true,
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
                            copyboard.board[row][4] = Some(new_king);
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

            // Replace the new kings with previous things if castled
            copyboard.make_move_only(*mv);

            let in_check = copyboard.in_check(piece.color.opposite());

            if in_check {
                mv.set_state(GameState::InCheck(piece.color.opposite()));
            }

            if check_next {
                let next_moves = copyboard.get_all_legal_moves(false);
                if next_moves.len() == 0 {
                    if in_check {
                        mv.set_state(GameState::Checkmate(piece.color.opposite()));
                    } else {
                        mv.set_state(GameState::Stalemate);
                    }
                }
            }

            piece_legal_moves.push(*mv);
        }
        piece_legal_moves
    }

    pub fn get_all_legal_moves(&self, check_next: bool) -> Vec<Move> {
        let color = self.next_player;
        self.board
            .into_par_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.into_par_iter()
                    .enumerate()
                    .flat_map(|(j, place)| {
                        if let Some(piece) = place
                            && piece.color == color
                        {
                            self.get_piece_legal_moves((i, j), check_next)
                        } else {
                            Vec::new()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn in_check(&mut self, color: Color) -> bool {
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
                Move::Promotion(p_move) => {
                    if let Some(piece) = p_move.capture {
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
        self.make_move_helper(mv, true);
    }

    fn make_move_helper(&mut self, mv: Move, check_next: bool) {
        self.make_move_only(mv);
        self.state = mv.get_state();

        // Add move to history
        self.move_history.push(mv);

        // Change color
        self.next_player = self.next_player.opposite();
        if check_next {
            self.next_legal_moves = self.get_all_legal_moves(check_next);
        } else {
            self.next_legal_moves = self.get_all_moves(self.next_player);
        }
    }

    fn make_move_only(&mut self, mv: Move) {
        match mv {
            Move::Normal(normal_move) => {
                self.make_normal_move(normal_move);
            }
            Move::Castles(castles_move) => {
                self.make_castles_move(castles_move);
            }
            Move::EnPassant(ep_move) => {
                self.make_enpassant_move(ep_move);
            }
            Move::Promotion(pr_move) => {
                self.make_promotion_move(pr_move);
            }
        }
    }

    fn make_normal_move(&mut self, mv: NormalMove) {
        self.board[mv.from_position.0][mv.from_position.1] = None;
        let mut pc = mv.piece;
        pc.has_moved = true;
        self.board[mv.to_position.0][mv.to_position.1] = Some(pc);
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
