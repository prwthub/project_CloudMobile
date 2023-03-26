use actix_web::web;
use crate::handlers::teacher_update_handler::put_teacher_update;

// teacher update [put]
// (/teacher/update)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(put_teacher_update);
}