use actix_web::{web, get, Responder, HttpResponse};

// grade [get]
// (/grade)
#[get("/grade")]
async fn get_grade() -> impl Responder {
    
    HttpResponse::Ok().json("grade")
}