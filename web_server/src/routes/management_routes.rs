use actix_web::web;
use crate::handlers::grade_handler::get_grade;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_grade);
}