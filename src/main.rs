mod services;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:3001");

    HttpServer::new(move || App::new().configure(services::config))
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}
