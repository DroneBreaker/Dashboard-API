use actix_web::{self, get, App, HttpResponse, HttpServer, Responder};
use user::user_routes::all_users;

mod db;
mod user;

#[get("/")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Apex Dashboard")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(login)
        .service(all_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
