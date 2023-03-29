use actix_web::{post, HttpResponse, Responder, delete, get};
use serde_json::json;

#[get("/manage/user")]
pub async fn view_user_score() -> impl Responder {
    let response_body = json!(
        {
        "massage": "List of the user",
        "user": [
          {
            "us_name": "Loki",
            "us_score": "50"
          },
          {
            "us_name": "Lola",
            "us_score": "60"
          },
          {
            "us_name": "Lucy",
            "us_score": "79"
          }
        ]
      }
    );

    HttpResponse::Ok().json(response_body)
}

#[delete("/manage/user")]
pub async fn delete_user() -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[post("/manage/course/{id}")]
pub async fn upload_course() -> impl Responder {
    HttpResponse::Created().json("Upload complete")
}

#[delete("/manage/course/{id}")]
pub async fn delete_course() -> impl Responder {
    HttpResponse::NoContent().finish()
}