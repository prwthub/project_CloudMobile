use actix_web::web;
use crate::handlers::login_handler::post_login;

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(post_login);
}

// use actix_web::web;
// use crate::handlers::login_handler::login;

// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/login")
//             .route(web::post().to(login_handler::login))
//     );
// }