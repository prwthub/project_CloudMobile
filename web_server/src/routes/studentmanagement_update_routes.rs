use actix_web::web;
use crate::handlers::studentmanagement_update_handler::put_studentmanagement_update;

// student manage update [put]
// (/studentmanage/update)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(put_studentmanagement_update);
}