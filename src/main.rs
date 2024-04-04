use actix_web::{self, App, HttpServer};
use user::routes::all_users;

mod db;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(all_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
