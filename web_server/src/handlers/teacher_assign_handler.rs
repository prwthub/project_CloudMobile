use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// teacher assign [post]
// (/teacher/assign)
#[get("/teacher/assign")]
async fn post_teacher_assign() -> impl Responder {
    
    HttpResponse::Ok().json("teacher assign")
}