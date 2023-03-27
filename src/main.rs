use actix_web::{App, HttpServer, middleware};
use env_logger::Env;

pub mod routes;
mod handlers;
mod models;
use crate::routes::course_routes;
use crate::routes::auth_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(course_routes::config)
            .configure(auth_routes::config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
