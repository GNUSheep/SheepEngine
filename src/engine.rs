use chess::{Board, MoveGen, Color, BoardStatus, Game, GameResult};
use std::str::FromStr;

static CHESS_PIECES_VALUE: [i32; 5] = [100, 300, 300, 500, 900];

fn checkmate_check(mut alpha: i32, beta: i32, board: Board, fen: &str) -> i32 {
    let evaluation = evaluate_position(fen, board);
    if evaluation >= beta {
        return beta
    }
    if alpha <= evaluation {
        alpha = evaluation;
    }

    let mut moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);

    for target in moves {
        let new_board = board.make_move_new(target);
        let out = -checkmate_check(-beta, -alpha, new_board, new_board.to_string().as_str());

        if beta <= out {
            return beta
        }
        if alpha <= evaluation {
            alpha = evaluation;
        }
    }
    
    return alpha
}

fn search(depth: i32, fen: &str, board: Board, mut alpha: i32, beta: i32) -> (i32, String) {
    if depth == 0 {
        //return (checkmate_check(alpha, beta, board, fen), "".to_string())
        return (evaluate_position(fen, board), "".to_string())
    }

    let moves = MoveGen::new_legal(&board);

    if moves.len() == 0 {
        let game: Game = Game::from_str(fen).expect("Valid FEN");
        let status = game.result().unwrap();
//
    //    if status == GameResult::WhiteCheckmates {
    //        return (-900000000, "".to_string())
    //    }
//
        if status == GameResult::BlackCheckmates {
            return (2000000000, "checkmate".to_string())
        }
//
    //    return (0, "".to_string())
    }

    let mut best_move = "".to_string();

    for move_element in moves {
        let new_board = board.make_move_new(move_element);
        //println!("{}, {}, {}, {}", move_element.to_string(), alpha, beta, new_board.to_string());
        let (out, check_move) = search(depth - 1, new_board.to_string().as_str(), new_board, -beta, -alpha);

        if check_move == "checkmate" {
            println!("{}", move_element.to_string());
            alpha = -out;
            best_move = move_element.to_string();
            return (alpha, best_move)
        }

        if beta <= -out {
            return (beta, "".to_string())
        }

        if alpha < -out {
            alpha = -out;
            best_move = move_element.to_string();
        }
    }

    return (alpha, best_move);
}

fn evaluate_position(fen: &str, board: Board) -> i32 {
    let fen_splited = fen.split(" ").collect::<Vec<_>>()[0].split("/").collect::<Vec<_>>();

    let mut black_evaluation = 0;
    let mut white_evaluation = 0;
    for row in fen_splited.iter() {
        for piece in row.chars() {
            match piece {
                'p' => {
                    black_evaluation += CHESS_PIECES_VALUE[0];           
                },
                'P' => {
                    white_evaluation += CHESS_PIECES_VALUE[0];  
                }
                'n' => {
                    black_evaluation += CHESS_PIECES_VALUE[1];           
                },
                'N' => {
                    white_evaluation += CHESS_PIECES_VALUE[1];  
                }
                'b' => {
                    black_evaluation += CHESS_PIECES_VALUE[2];           
                },
                'B' => {
                    white_evaluation += CHESS_PIECES_VALUE[2];  
                }
                'r' => {
                    black_evaluation += CHESS_PIECES_VALUE[3];           
                },
                'R' => {
                    white_evaluation += CHESS_PIECES_VALUE[3];  
                }
                'q' => {
                    black_evaluation += CHESS_PIECES_VALUE[4];           
                },
                'Q' => {
                    white_evaluation += CHESS_PIECES_VALUE[4];  
                }
                _ => (),
            }
        }
    }

    let game: Game = Game::from_str(fen).expect("Valid FEN");
    let status = game.result();
    if status != None {
        if status.unwrap() == GameResult::BlackCheckmates {
            black_evaluation = 100000000;
        }
    }

    if board.side_to_move() == Color::Black {
        return -1 * (white_evaluation - black_evaluation)
    }else {
        return white_evaluation - black_evaluation
    }
}

pub fn make_move(fen: &str) -> String {
    let chess_board = Board::from_str(fen).unwrap();

    let (out, best_move) = search(2, fen, chess_board, -100000000, 100000000);
    println!("Best: {}, {}", out, best_move);

    return best_move
}