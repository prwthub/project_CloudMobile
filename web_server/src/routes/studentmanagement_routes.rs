use actix_web::web;
use crate::handlers::studentmanagement_handler::get_studentmanagement;

// student manage [get]
// (/studentmanage)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_studentmanagement);
}