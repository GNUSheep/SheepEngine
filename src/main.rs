use actix_web_static_files::ResourceFiles;
use actix_web::{App, HttpServer};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let generated = generate();
        App::new().service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}