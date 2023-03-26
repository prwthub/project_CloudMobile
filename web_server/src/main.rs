use actix_web::{App, HttpServer};
//use serde::Deserialize;

pub mod routes;
use crate::routes::{register_routes, login_routes, grade_routes, management_routes};
use crate::routes::{studentmanagement_routes, studentmanagement_update_routes, studentmanagement_delete_routes};
use crate::routes::{teacher_routes, teacher_update_routes, teacher_assign_routes};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
        App::new()
            .configure(register_routes::config)
            .configure(login_routes::config)
            .configure(grade_routes::config)
            .configure(management_routes::config)
            
            .configure(studentmanagement_routes::config)
            .configure(studentmanagement_update_routes::config)
            .configure(studentmanagement_delete_routes::config)
            
            .configure(teacher_routes::config)
            .configure(teacher_update_routes::config)
            .configure(teacher_assign_routes::config)
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await
}
