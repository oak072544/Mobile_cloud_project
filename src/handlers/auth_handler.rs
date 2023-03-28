use actix_web::{web, post, HttpResponse, Responder};
use serde_json::json;


use crate::models::user::User;

#[post("/login")]
pub async fn login(user: web::Json<User>) -> impl Responder {
    // perform authentication
    if user.username == "username" && user.password == "password" {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[post("/logout")]
pub async fn logout() -> impl Responder {
    // perform logout
    HttpResponse::Ok().finish()
}

#[post("/signup")]
pub async fn signup(user: web::Json<User>) -> impl Responder {
    // Implement user registration logic here, for example, by adding the user to a database
    let response = format!("User with username {} successfully registered!",user.username);
    let response_body = json!(response);
    HttpResponse::Ok().json(response_body)
}