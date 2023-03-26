use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// student manage update [put]
// (/studentmanage/update)
#[get("/studentmanagement/update")]
async fn put_studentmanagement_update() -> impl Responder {
    
    HttpResponse::Ok().json("studentmanagement update")
}