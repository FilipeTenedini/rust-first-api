mod model;
mod schema;
mod services;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:3001");

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to the database");
            pool
        }
        Err(e) => {
            println!("Error connecting to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
    })
    .bind("127.0.0.1:3001")?
    .run()
    .await
}
