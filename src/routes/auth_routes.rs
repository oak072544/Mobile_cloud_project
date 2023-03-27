use actix_web::web;
use crate::handlers::auth_handler::{login, logout};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(login)
        .service(logout);
}
