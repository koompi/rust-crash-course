pub mod auth;
pub mod mw;
use actix_web::{App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().wrap(mw::Authorization))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
