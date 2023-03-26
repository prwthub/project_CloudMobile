use actix_web::web;
use crate::handlers::teacher_assign_handler::post_teacher_assign;

// teacher assign [post]
// (/teacher/assign)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(post_teacher_assign);
}