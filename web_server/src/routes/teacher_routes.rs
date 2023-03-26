use actix_web::web;
use crate::handlers::teacher_handler::get_teacher;

// teacher [get]
// (/teacher)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_teacher);
}