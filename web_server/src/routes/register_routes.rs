use actix_web::web;
use crate::handlers::register_handler::post_register;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(post_register);
}