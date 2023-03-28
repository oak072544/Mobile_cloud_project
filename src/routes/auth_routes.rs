use actix_web::web;
use crate::handlers::auth_handler::{login, logout, signup};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(logout)
        .service(
        web::resource("/signup")
            .route(web::post().to(signup)));
}
