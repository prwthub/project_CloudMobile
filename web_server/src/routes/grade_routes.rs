use actix_web::web;
use crate::handlers::management_handler::get_management;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_management);
}