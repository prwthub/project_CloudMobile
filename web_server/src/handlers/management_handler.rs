use actix_web::{web, get, Responder, HttpResponse};

// management [get]
// (/management)
#[get("/management")]
async fn get_management() -> impl Responder {
    
    HttpResponse::Ok().json("management")
}