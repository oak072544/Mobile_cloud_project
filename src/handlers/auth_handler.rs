use actix_web::{web, post, HttpResponse, Responder};


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
