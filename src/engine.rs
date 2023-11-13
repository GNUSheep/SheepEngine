use chess::{Board, ChessMove, Color, Game, GameResult, MoveGen, Piece, Square};
use std::str::FromStr;
use std::time::{Instant};

mod piece_tables;

static CHESS_PIECES_VALUE: [i32; 5] = [100, 320, 330, 500, 900];

struct Ordering {
    killer_moves: Vec<Vec<ChessMove>>,
}

impl Ordering {
    unsafe fn init() -> Self {   
        let killer_moves = vec![vec![ChessMove::new(Square::new(0), Square::new(0), Some(Piece::Queen)); 2]; 5];

        Self{killer_moves}
    }

    fn update(&mut self, killer_move: ChessMove, depth: usize) {
        self.killer_moves[depth][1] = self.killer_moves[depth][0];
        self.killer_moves[depth][0] = killer_move;
    }
}

//fn checkmate_check(mut alpha: f32, beta: f32, board: Board, fen: &str) -> f32 {
//    let evaluation = evaluate_position(fen, board);
//    if evaluation >= beta {
//        return beta;
//    }
//
//    if evaluation < alpha - 900.0 {
//        return alpha
//    }
//
//    if alpha < evaluation {
//        alpha = evaluation;
//    }
//
//    let mut moves = MoveGen::new_legal(&board);
//    let targets = board.color_combined(!board.side_to_move());
//    moves.set_iterator_mask(*targets);
//
//    let mut moves: Vec<_> = moves.collect();
//    moves.sort_by(|a, b| get_move_value(board, b).cmp(&get_move_value(board, a)));
//
//    for target in moves {
//        let new_board = board.make_move_new(target);
//        let out = -checkmate_check(-beta, -alpha, new_board, new_board.to_string().as_str());
//
//        if beta <= out {
//            return beta;
//        }
//        if alpha < out {
//            alpha = out;
//        }
//    }
//
//    return alpha;
//}

fn search_root(depth: i32, board: Board, mut order_values: Ordering) -> (f32, String) {
    let moves = order_moves(board, &mut order_values, depth as usize);

    let mut best_value = f32::INFINITY;

    let mut best_move = " ".to_string();

    for move_element in moves {
        let new_board = board.make_move_new(move_element);

        let out = minmax(
            depth - 1,
            new_board.to_string().as_str(),
            new_board,
            f32::NEG_INFINITY,
            f32::INFINITY,
            &mut order_values,
            true,
        );

        if out <= best_value {
            best_value = out;
            best_move = move_element.to_string();
        }
    }

    return (best_value, best_move);
}

fn get_piece_type_value(piece: Piece) -> i32 {
    if piece == Piece::Pawn {
        return CHESS_PIECES_VALUE[0];
    }
    if piece == Piece::Knight {
        return CHESS_PIECES_VALUE[1];
    }
    if piece == Piece::Bishop {
        return CHESS_PIECES_VALUE[2];
    }
    if piece == Piece::Rook {
        return CHESS_PIECES_VALUE[3];
    }
    if piece == Piece::Queen {
        return CHESS_PIECES_VALUE[4];
    }

    return f32::INFINITY as i32;
}

fn get_piece_value(piece: Piece) -> usize {
    if piece == Piece::Pawn {
        return 5
    }
    if piece == Piece::Knight {
        return 4
    }
    if piece == Piece::Bishop {
        return 3
    }
    if piece == Piece::Rook {
        return 2
    }
    if piece == Piece::Queen {
        return 1
    }else{
        return 0
    }
}

fn get_move_value(board: Board, move_element: &ChessMove, order_values: &mut Ordering, depth: usize) -> i32 {
    // 1. Captures
    // 2. Promotions
    // 3. Killer Moves

    let move_guess = 0;

    let piece_moving = board.piece_on(move_element.get_source());
    let to = move_element.get_dest();

    if board.piece_on(to) != None {
        let from_value = get_piece_value(piece_moving.unwrap());
        let capture_value = get_piece_value(board.piece_on(to).unwrap());

        return piece_tables::MVV_LVA[from_value][capture_value] + 10000
    }else{
        if order_values.killer_moves[depth][0] == *move_element {
            return 9000
        }else if order_values.killer_moves[depth][1] == *move_element {
            return 8000
        }
    }

    //Promotion
    if piece_moving.unwrap() == Piece::Pawn {
        if to.get_rank() == Rank::First || to.get_rank() == Rank::Eighth {
            move_guess += 100000;
        }
    }

    return move_guess;
}

fn order_moves(board: Board, order_values: &mut Ordering, depth: usize) -> Vec<ChessMove> {
    let mut moves: Vec<_> = MoveGen::new_legal(&board).collect();

    moves.sort_by(|a, b| get_move_value(board, a, order_values, depth).cmp(&get_move_value(board, b, order_values, depth)));

    return moves;
}

fn minmax(
    depth: i32,
    fen: &str,
    board: Board,
    mut alpha: f32,
    mut beta: f32,
    order_values: &mut Ordering,
    maximizing: bool,
) -> f32 {
    //println!("1");
   // println!("{}", order_values.killer_moves[depth as usize][0].to_string());

    if depth == 0 {
        return evaluate_position(fen, board)
        //return checkmate_check(alpha, beta, board, fen)
    }

    let moves = order_moves(board, order_values, depth as usize);

    if moves.len() == 0 {
        return evaluate_position(fen, board);
    }

    if maximizing {
        for move_element in moves {
            let new_board = board.make_move_new(move_element);

            let out = minmax(
                depth - 1,
                new_board.to_string().as_str(),
                new_board,
                alpha,
                beta,
                order_values,
                false,
            );
            if out >= beta {
                let IsCapture = board.piece_on(move_element.get_dest());
                if IsCapture == None {
                    order_values.update(move_element, depth as usize);
                }

                return beta
            }
            if out > alpha {
                alpha = out;
            }
        }
        return alpha
    } else {
        for move_element in moves {
            let new_board = board.make_move_new(move_element);

            let out = minmax(
                depth - 1,
                new_board.to_string().as_str(),
                new_board,
                alpha,
                beta,
                order_values,
                true,
            );
            if out <= alpha {
                let IsCapture = board.piece_on(move_element.get_dest());
                if IsCapture == None {
                    order_values.update(move_element, depth as usize);
                }

                return alpha
            }
            if out < beta {
                beta = out;
            }
        }
        return beta
    }
}

fn evaluate_position(fen: &str, board: Board) -> f32 {
    println!("1");

    let fen_splited = fen.split(" ").collect::<Vec<_>>()[0]
        .split("/")
        .collect::<Vec<_>>();

    let mut black_evaluation = 0;
    let mut white_evaluation = 0;

    let mut square_index = 0;
    for row in fen_splited.iter() {
        for piece in row.chars() {
            match piece {
                'p' => {
                    black_evaluation += CHESS_PIECES_VALUE[0];
                    black_evaluation += piece_tables::PAWN_B[square_index];
                }
                'P' => {
                    white_evaluation += CHESS_PIECES_VALUE[0];
                    white_evaluation += piece_tables::PAWN[square_index];
                }
                'n' => {
                    black_evaluation += CHESS_PIECES_VALUE[1];
                    black_evaluation += piece_tables::KNIGHT_B[square_index];
                }
                'N' => {
                    white_evaluation += CHESS_PIECES_VALUE[1];
                    white_evaluation += piece_tables::KNIGHT[square_index];
                }
                'b' => {
                    black_evaluation += CHESS_PIECES_VALUE[2];
                    black_evaluation += piece_tables::BISHOP_B[square_index];
                }
                'B' => {
                    white_evaluation += CHESS_PIECES_VALUE[2];
                    white_evaluation += piece_tables::BISHOP[square_index];
                }
                'r' => {
                    black_evaluation += CHESS_PIECES_VALUE[3];
                    black_evaluation += piece_tables::ROOK_B[square_index];
                }
                'R' => {
                    white_evaluation += CHESS_PIECES_VALUE[3];
                    white_evaluation += piece_tables::ROOK[square_index];
                }
                'q' => {
                    black_evaluation += CHESS_PIECES_VALUE[4];
                    black_evaluation += piece_tables::QUEEN_B[square_index];
                }
                'Q' => {
                    white_evaluation += CHESS_PIECES_VALUE[4];
                    white_evaluation += piece_tables::QUEEN[square_index];
                }
                _ => (),
            }
            square_index += 1;
        }
    }

    let game: Game = Game::from_str(fen).expect("Valid FEN");
    let status = game.result();
    if status != None {
        if status.unwrap() == GameResult::BlackCheckmates {
            black_evaluation = 200000000;
        }

        if status.unwrap() == GameResult::WhiteCheckmates {
            white_evaluation = 200000000;
        }
    }

    if board.side_to_move() == Color::Black {
        return (-1 * (white_evaluation - black_evaluation)) as f32;
    } else {
        return (white_evaluation - black_evaluation) as f32;
    }
}

pub fn make_move(fen: &str) -> String {
    let start = Instant::now();
    let chess_board = Board::from_str(fen).unwrap();
    //"1nbqk2r/6pp/8/r7/3p4/3p1KP1/5P1P/4q3 b k - 1 32"

    let mut order_values = unsafe {Ordering::init()};

    let (out, best_move) = search_root(4, chess_board, order_values);
    println!("Best: {}, {}", out, best_move);
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);

    return best_move;
}
