use crate::chess_engine::{board::*, moves::*, piece::*, read_fen_string::check_fen_str};

pub mod chess_engine;

fn main() {
    let mut my_board = chess_engine::board::Board::new_standard_board();

    // let tic = SystemTime::now();
    // let moves = my_board.get_all_legal_moves(Color::Black, true);
    // let duration = SystemTime::now().duration_since(tic).unwrap();

    // println!("{:?}", moves);
    // println!("{:?}", moves.len());

    // println!("{:?}", duration);

    let my_fen = "rnbqkbnr/1ppppp1p/p7/4P3/5Pp1/3P4/PPP3PP/RNBQKBNR";
    println!("{:?}", check_fen_str(my_fen));

    match my_board.from_fen_str(my_fen) {
        Ok(()) => {
            println!("{:?}", my_board.board);
        }
        Err(e) => {
            println!("{}", e);
        }
    };
    my_board.last_move = Move::Normal(NormalMove {
        piece: Piece {
            color: Color::White,
            piece_type: PieceType::Pawn,
            has_moved: false,
        },
        from_position: (1, 5),
        to_position: (3, 5),
        capture: None,
    });
    let allMoves = my_board.get_piece_moves(
        (3, 6),
        &Piece {
            color: Color::Black,
            piece_type: PieceType::Pawn,
            has_moved: true,
        },
    );
    my_board.make_move(Move::EnPassant(EnPassantMove { from_position: (3, 6), to_position: (2, 5), pawn_capture_position: (3, 5) }));
    for row in my_board.board{
        println!("{:?}", row);
    }
}
