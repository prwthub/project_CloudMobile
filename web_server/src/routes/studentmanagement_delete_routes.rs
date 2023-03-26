use actix_web::web;
use crate::handlers::studentmanagement_delete_handler::del_studentmanagement_delete;

// student manage delete [delete]
// (/studentmanage/delete)
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(del_studentmanagement_delete);
}