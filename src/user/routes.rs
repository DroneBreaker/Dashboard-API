use actix_web::{delete, get, post, HttpResponse, Responder};

#[get("/users")]
pub async fn all_users() -> impl Responder {
    HttpResponse::Ok().body("All users here")
}

#[post("/users")]
pub async fn add_user() -> impl Responder {
    HttpResponse::Ok().body("User created successfully")
}

#[get("/user/{id}")]
pub async fn get_user_by_id() -> impl Responder {
    HttpResponse::Ok().body("User id is available")
}

#[delete("/user/{id}")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::Ok().body("User deleted successfully")
}