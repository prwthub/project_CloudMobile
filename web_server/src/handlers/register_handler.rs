use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// register [post]
// (/register)
#[get("/register")]
async fn post_register() -> impl Responder {
    
    HttpResponse::Ok().json("register")
}