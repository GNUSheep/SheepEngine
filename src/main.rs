use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use std::io::stdin;
use std::str::FromStr;
use chess::Game;
use chess::ChessMove;

mod engine;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[post("/")]
async fn get_fen(body: String) -> impl Responder {
    let respond = engine::make_move(body.as_str());

    HttpResponse::Ok().body(respond)
}

#[actix_web::main]
async fn gui() -> std::io::Result<()> {
    HttpServer::new(|| {
        let generated = generate();
        App::new()
            .service(get_fen)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn uci() {
    println!("id name SheepEngine");
    println!("id author GNUSheep");
    println!("uciok");
    
    let mut game = Game::new();
    while true {
        let mut command = String::new();
        stdin().read_line(&mut command).expect("Error invalid input");

        let command_splited: Vec<&str> = command.trim().split(" ").collect();

        match command_splited[0] {
            "isready" => println!("readyok"),
            "ucinewgame" => (),
            "position" => {
                game = Game::new();
                if command_splited[1] == "startpos" {
                    for move_element in command_splited.iter().skip(3) {
                        game.make_move(ChessMove::from_str(move_element).unwrap());
                    }
                }else if command_splited[1] == "fen" {
                    let (command, fen) = command.trim().split_at(13);
                    game = Game::from_str(fen).expect("Valid FEN");
                }
            },
            "go" => {
                let best_move = engine::make_move(game.current_position().to_string().as_str());
                println!("bestmove {}", best_move.to_string());
            },
            "quit" => break,
            _ => (),
        }
    }
}

fn main() {
    //println!("Sheep Engine - Enter uci or gui to continue");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).expect("Valid input");

    if user_input.trim().eq("gui") {
        println!("localhost:8080");
        let _ = gui();
    }else if user_input.trim().eq("uci") {
        uci();
    }
}