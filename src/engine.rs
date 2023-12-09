use chess::{Board, ChessMove, Color, Game, GameResult, MoveGen, Piece, Square};
use std::str::FromStr;
use std::time::{Instant};

mod piece_tables;

static CHESS_PIECES_VALUE: [i32; 5] = [100, 320, 330, 500, 900];

struct Ordering {
    killer_moves: Vec<Vec<ChessMove>>,
    best_move: ChessMove,
    nodes: i32,
}

impl Ordering {
    unsafe fn init() -> Self {   
        let killer_moves = vec![vec![ChessMove::new(Square::new(0), Square::new(0), Some(Piece::Queen)); 2]; 100];
        let best_move = ChessMove::new(Square::new(0), Square::new(0), Some(Piece::Queen));

        let nodes = 0;

        Self{killer_moves, best_move, nodes}
    }

    fn update_killer(&mut self, killer_move: ChessMove, depth: usize) {
        self.killer_moves[depth][1] = self.killer_moves[depth][0];
        self.killer_moves[depth][0] = killer_move;
    }

    fn update_best(&mut self, best_move: ChessMove) {
        self.best_move = best_move;
    }
}

//fn checkmate_check(mut alpha: f32, beta: f32, board: Board, fen: &str, order_values: &mut Ordering) -> f32 {
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
//    if board.side_to_move() == Color::Black {
//        moves.sort_by(|a, b| get_move_value(board, a, order_values, 0).cmp(&get_move_value(board, b, order_values, 0)));
//    }else{
//        moves.sort_by(|b, a| get_move_value(board, a, order_values, 0).cmp(&get_move_value(board, b, order_values, 0)));
//    }
//
//    for target in moves {
//        let new_board = board.make_move_new(target);
//        let out = -checkmate_check(-beta, -alpha, new_board, new_board.to_string().as_str(), order_values);
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
//
fn search_root(board: Board, mut order_values: Ordering) -> (f32, String) {
    let start = Instant::now();
    let mut best_value = 0.0;
    
    for depth in 1..=4 {
        order_values.nodes = 0;

        best_value = f32::NEG_INFINITY; 
        let maximizing = board.side_to_move() == Color::White;
        if !maximizing {
            best_value = f32::INFINITY
        }
        
        let moves = order_moves(board, &mut order_values, depth as usize);
        for move_element in moves {
            let new_board = board.make_move_new(move_element);
    
            let game: Game = Game::from_str(new_board.to_string().as_str()).expect("Valid FEN");
            let out;

            if game.can_declare_draw() {
                out = 0.0;
            }else{
                out = minmax(
                    depth - 1,
                    new_board.to_string().as_str(),
                    new_board,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                    &mut order_values,
                    !maximizing,
                );
            }
    
            if !maximizing && out <= best_value {
                best_value = out;
                order_values.update_best(move_element);
            }else if maximizing && out >= best_value {
                best_value = out;
                order_values.update_best(move_element);
            }
        }
        println!("Depth: {}, nodes: {}, time: {:?}", depth, order_values.nodes, start.elapsed());
    }


    return (best_value, order_values.best_move.to_string());
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
    // 1. Best move iterration deeping
    // 2. Captures
    // 3. Promotions
    // 4. Killer Moves

    let mut move_guess = 0;

    if move_element == &order_values.best_move {
        move_guess += 30000
    }

    let piece_moving = board.piece_on(move_element.get_source());
    let to = move_element.get_dest();

    if board.piece_on(to) != None {
        let from_value = get_piece_value(piece_moving.unwrap());
        let capture_value = get_piece_value(board.piece_on(to).unwrap());

        move_guess += piece_tables::MVV_LVA[from_value][capture_value] + 10000
    }else{
        if order_values.killer_moves[depth][0] == *move_element {
            move_guess += 9000
        }else if order_values.killer_moves[depth][1] == *move_element {
            move_guess += 8000
        }
    }

    if board.side_to_move() == Color::Black {
        return -move_guess
    }
    return move_guess
}

fn order_moves(board: Board, order_values: &mut Ordering, depth: usize) -> Vec<ChessMove> {
    let mut moves: Vec<_> = MoveGen::new_legal(&board).collect();

    if board.side_to_move() == Color::Black {
        moves.sort_by(|a, b| get_move_value(board, a, order_values, depth).cmp(&get_move_value(board, b, order_values, depth)));
    }else{
        moves.sort_by(|b, a| get_move_value(board, a, order_values, depth).cmp(&get_move_value(board, b, order_values, depth)));
    }

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

   order_values.nodes += 1;

    if depth == 0 {
        return evaluate_position(fen, board)
        //return checkmate_check(alpha, beta, board, fen, order_values)
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
                    order_values.update_killer(move_element, depth as usize);
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
                    order_values.update_killer(move_element, depth as usize);
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

fn evaluatePawns(board: Board) -> (i32, i32) {
    let pawns = board.pieces(Piece::Pawn);

    let mut w_eval = 0;
    let mut b_eval = 0;

    for pawn in pawns.into_iter() {
        let mut left_to_prom: i32 = pawn.get_rank().to_index().try_into().unwrap();
        let color = board.color_on(pawn).unwrap();
        if color == Color::White {
            left_to_prom *= -1;
            left_to_prom += 7;
        }
    
        let mut square = pawn;
        let mut is_passed = true;
        for _ in 0..left_to_prom {
            if color == Color::White {
                square = square.up().unwrap();
            }else {
                square = square.down().unwrap();
            }
    
            // up
            if board.piece_on(square)!= None {
                // doubled
                if board.color_on(square) == Some(color) {
                    if color == Color::White {
                        w_eval -= 50;
                    }else {b_eval -= 50};       
                }
                is_passed = false;
            }

            // left 
            let square_left = square.left();
            if !square_left.is_none() {
                if board.piece_on(square_left.unwrap()) != None && board.color_on(square_left.unwrap()) == Some(!color) {
                    is_passed = false;
                }
            }

            let square_right = square.right();
            if !square_right.is_none() {
                if board.piece_on(square_right.unwrap()) != None && board.color_on(square_right.unwrap()) == Some(!color) {
                    is_passed = false;
                }
            }
        }

        if is_passed {
            if color == Color::White {
                w_eval += piece_tables::PASSED_PAWN_BONUS[left_to_prom as usize];
            }else{b_eval += piece_tables::PASSED_PAWN_BONUS[left_to_prom as usize]};
        }
    }

    return (w_eval, b_eval)
}

fn evaluate_position(fen: &str, board: Board) -> f32 {
    //println!("1");

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

    let (pawns_white_eval, pawns_black_eval) = evaluatePawns(board);
    white_evaluation += pawns_white_eval;
    black_evaluation += pawns_black_eval;

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

    return (white_evaluation - black_evaluation) as f32;
}

pub fn make_move(fen: &str) -> String {
    //let start = Instant::now();
    let chess_board = Board::from_str(fen).unwrap();
    //"1nbqk2r/6pp/8/r7/3p4/3p1KP1/5P1P/4q3 b k - 1 32"

    let mut order_values = unsafe {Ordering::init()};

    let (out, best_move) = search_root(chess_board, order_values);

    return best_move;
}
