use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:1004")?
    .run()
    .await
}