use actix_web::web;
use crate::handlers::course_handler::{get_course, get_all_courses, exercise};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_all_courses)
        .service(get_course)
        .service(exercise);
}
