use actix_web::{web, post, get, put, delete, Responder, HttpResponse};

// student manage delete [delete]
// (/studentmanage/delete)
#[get("/studentmanagement/delete")]
async fn del_studentmanagement_delete() -> impl Responder {
    
    HttpResponse::Ok().json("studentmanagement delete")
}