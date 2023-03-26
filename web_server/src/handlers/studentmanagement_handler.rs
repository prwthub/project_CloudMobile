use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// student manage [get]
// (/studentmanage)
#[get("/studentmanagement")]
async fn get_studentmanagement() -> impl Responder {
    
    HttpResponse::Ok().json("studentmanagement")
}