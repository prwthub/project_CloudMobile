use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// teacher [get]
// (/teacher)
#[get("/teacher")]
async fn get_teacher() -> impl Responder {
    
    HttpResponse::Ok().json("teacher")
}