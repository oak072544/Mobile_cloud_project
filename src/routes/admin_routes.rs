use actix_web::web;
use crate::handlers::admin_handler::{view_user_score, delete_user, upload_course,delete_course};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(view_user_score)
        .service(delete_user)
        .service(upload_course)
        .service(delete_course);
        
}
