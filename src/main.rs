use actix_web_static_files::ResourceFiles;
use actix_web::{App, HttpServer, HttpResponse, web, Responder, Error, post};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[post("/")]
async fn get_fen(body: String) -> impl Responder {
    println!("{}", body);

    HttpResponse::Ok().body("0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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