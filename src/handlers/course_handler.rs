use serde_json::json;

use actix_web::{get, Responder, HttpResponse, web,post};
use crate::models::{database::Database, exercise::Exercise};

#[get("/courses")]
async fn get_all_courses() -> impl Responder {
    let database = Database::new(); // create a new instance of Database
    let courses = database.get_courses(); // pass it to the function

    HttpResponse::Ok().json(courses)
}

#[get("/courses/{id}")]
async fn get_course(course_id: web::Path<i32>) -> impl Responder {
    let database = Database::new(); // create a new instance of Database
    let course = database.get_course(course_id.into_inner()); // pass it to the function

    match course {
        Some(course) => HttpResponse::Ok().json(course),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/courses/{id}")]
pub async fn exercise(result: web::Json<Exercise>) -> impl Responder {
    // Implement user registration logic here, for example, by adding the user to a database
    let response = format!("Your score is {}",result.score);
    let response_body = json!(response);
    HttpResponse::Created().json(response_body)
}