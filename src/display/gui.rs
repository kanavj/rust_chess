use crate::{
    Display,
    game::{self, Game},
    moves::Move,
    piece::{Piece, PieceType},
};
use macroquad::prelude::*;
use std::{cmp, collections::HashMap};

pub struct GUIDisplayer {
    rows: usize,
    cols: usize,
    sw: f32,
    sh: f32,
    square_size: f32,
    offset_x: f32,
    offset_y: f32,
    selected_piece_square: Option<(usize, usize)>,
    background_tex: Texture2D,
    piece_texs: HashMap<(PieceType, game::Color), Texture2D>,
}

const T_ORANGE: Color = Color {
    r: 1.,
    g: 0.63,
    b: 0.,
    a: 0.5,
};

const T_GREY: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 0.5,
};

const OLIVE: Color = Color {
    r: 119. / 255.,
    g: 149. / 255.,
    b: 86. / 255.,
    a: 1.,
};

impl Display for GUIDisplayer {
    fn display_message(&self, message: String) {}

    async fn display(&mut self, game: &Game) {
        if (self.sw != screen_width()) || (self.sh != screen_height()) {
            (self.sw, self.sh) = (screen_width(), screen_height());
            self.recalculate();
        }

        self.draw_background();

        self.draw_board();

        self.draw_board_pieces(game);

        self.piece_selection(game);

        self.draw_piece_selection(game);

        next_frame().await;
    }

    fn user_input(&mut self, game: &Game) -> Option<Move> {
        if is_mouse_button_pressed(MouseButton::Left) {
            match self.selected_piece_square {
                None => {
                    self.try_to_select(game);
                }
                Some((i_sel, j_sel)) => {
                    if let Some((i_cand, j_cand)) = self.get_mouse_position_pair() {
                        if let Some(mv) = game
                            .moves_from_square((i_sel, j_sel))
                            .iter()
                            .find(|mv| mv.get_to_position() == (i_cand, j_cand))
                        {
                            self.selected_piece_square = None;
                            return Some(*mv);
                        }

                        self.try_to_select(game);
                    }
                }
            }
        }
        return None;
    }
}

impl GUIDisplayer {
    pub async fn new(rows: usize, cols: usize) -> Self {
        let background_tex = load_texture("./assets/background.png").await.unwrap();
        let mut piece_text_map: HashMap<(PieceType, game::Color), Texture2D> = HashMap::new();

        import_pieces(&mut piece_text_map).await;

        GUIDisplayer {
            rows,
            cols,
            sw: 0.,
            sh: 0.,
            square_size: 0.,
            offset_x: 0.,
            offset_y: 0.,
            selected_piece_square: None,
            background_tex: background_tex,
            piece_texs: piece_text_map,
        }
    }

    fn draw_board(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let color = if (i + j) % 2 == 0 { OLIVE } else { WHITE };
                let (x, y) = self.board_square_pixels(i, j);
                draw_rectangle(x, y, self.square_size, self.square_size, color);
            }
        }
    }

    fn draw_background(&self) {
        draw_texture_ex(
            &self.background_tex,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: self.sw,
                    y: self.sh,
                }),
                ..Default::default()
            },
        );
    }

    fn draw_board_pieces(&self, game: &Game) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                // Don't draw the selected piece

                if let Some((selected_i, selected_j)) = self.selected_piece_square {
                    if selected_i == i && selected_j == j {
                        continue;
                    }
                }
                if let Some(pc) = game.board[i][j] {
                    let (x, y) = self.board_square_pixels(i, j);
                    self.draw_piece(&pc, x, y);
                }
            }
        }
    }

    fn draw_piece(&self, piece: &Piece, x: f32, y: f32) {
        let text = self
            .piece_texs
            .get(&(piece.piece_type, piece.color))
            .expect("Piece not loaded");
        draw_texture_ex(
            text,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: self.square_size,
                    y: self.square_size,
                }),
                ..Default::default()
            },
        );
    }

    fn draw_piece_selection(&self, game: &Game) {
        if let Some((i, j)) = self.selected_piece_square {
            self.highlight_square(i, j, T_ORANGE);

            // draw selected piece
            let (x, y) = self.board_square_pixels(i, j);

            self.draw_piece(&game.board[i][j].expect(""), x, y);

            for mv in game.moves_from_square((i, j)) {
                let to_postion = mv.get_to_position();
                let (x, y) = self.board_square_pixels(to_postion.0, to_postion.1);
                draw_circle(
                    x + self.square_size / 2.,
                    y + self.square_size / 2.,
                    self.square_size / 6.,
                    T_GREY,
                )
            }
        }
    }

    fn try_to_select(&mut self, game: &Game) {
        if let Some((i_cand, j_cand)) = self.get_mouse_position_pair() {
            if !game.moves_from_square((i_cand, j_cand)).is_empty() {
                self.selected_piece_square = Some((i_cand, j_cand));
            }
        }
    }

    fn recalculate(&mut self) {
        self.square_size = (self.sw / self.cols as f32).min((self.sh / self.rows as f32));

        (self.offset_x, self.offset_y) = (
            (self.sw - self.square_size * self.cols as f32) / 2.,
            (self.sh - self.square_size * self.rows as f32) / 2.,
        );
    }

    fn highlight_square(&self, i: usize, j: usize, color: macroquad::color::Color) {
        let (x, y) = self.board_square_pixels(i, j);
        draw_rectangle(x, y, self.square_size, self.square_size, color);
    }

    fn piece_selection(&mut self, game: &Game) {
        if is_mouse_button_pressed(MouseButton::Left) && self.selected_piece_square.is_none() {
            if let Some((i_cand, j_cand)) = self.get_mouse_position_pair() {
                if !game.moves_from_square((i_cand, j_cand)).is_empty() {
                    self.selected_piece_square = Some((i_cand, j_cand));
                    return;
                }
            }
        }
    }

    fn get_mouse_position_pair(&self) -> Option<(usize, usize)> {
        let (mx, my) = mouse_position();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let (sq_x, sq_y) = self.board_square_pixels(i, j);
                if (sq_x <= mx && mx < sq_x + self.square_size)
                    && (sq_y <= my && my < sq_y + self.square_size)
                {
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    fn board_square_pixels(&self, i: usize, j: usize) -> (f32, f32) {
        (
            self.offset_x + j as f32 * self.square_size,
            self.offset_y + (self.rows - i - 1) as f32 * self.square_size,
        )
    }

    fn pixels_to_board_coords(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x - self.offset_x) / (self.sw - 2. * self.offset_x),
            (y - self.offset_y) / (self.sh - 2. * self.offset_y),
        )
    }

    fn board_coords_to_pixels(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x * (self.sw - 2. * self.offset_x) + self.offset_x,
            y * (self.sh - 2. * self.offset_y) + self.offset_y,
        )
    }
}

async fn import_pieces(map: &mut HashMap<(PieceType, game::Color), Texture2D>) {
    let pieces = [
        (
            PieceType::Pawn,
            game::Color::Black,
            "./assets/pawn_black.png",
        ),
        (
            PieceType::Rook,
            game::Color::Black,
            "./assets/rook_black.png",
        ),
        (
            PieceType::Bishop,
            game::Color::Black,
            "./assets/bishop_black.png",
        ),
        (
            PieceType::Knight,
            game::Color::Black,
            "./assets/knight_black.png",
        ),
        (
            PieceType::Queen,
            game::Color::Black,
            "./assets/queen_black.png",
        ),
        (
            PieceType::King,
            game::Color::Black,
            "./assets/king_black.png",
        ),
        (
            PieceType::Pawn,
            game::Color::White,
            "./assets/pawn_white.png",
        ),
        (
            PieceType::Rook,
            game::Color::White,
            "./assets/rook_white.png",
        ),
        (
            PieceType::Bishop,
            game::Color::White,
            "./assets/bishop_white.png",
        ),
        (
            PieceType::Knight,
            game::Color::White,
            "./assets/knight_white.png",
        ),
        (
            PieceType::Queen,
            game::Color::White,
            "./assets/queen_white.png",
        ),
        (
            PieceType::King,
            game::Color::White,
            "./assets/king_white.png",
        ),
    ];
    for (piece_type, color, path) in pieces {
        map.insert((piece_type, color), load_texture(path).await.unwrap());
    }
}
