use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// teacher update [put]
// (/teacher/update)
#[get("/teacher/update")]
async fn put_teacher_update() -> impl Responder {
    
    HttpResponse::Ok().json("teacher update")
}