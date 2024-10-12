use crate::api::ApiDoc;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let openapi = ApiDoc::openapi();

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_patients)
            .service(handlers::add_patient)
            .service(handlers::update_patient)
            .service(handlers::delete_patient)
            .service(handlers::get_doctors)
            .service(handlers::add_doctor)
            .service(handlers::update_doctor)
            .service(handlers::delete_doctor)
            .service(handlers::get_tickets)
            .service(handlers::add_ticket)
            .service(handlers::update_ticket)
            .service(handlers::delete_ticket)
            .service(handlers::get_schedule)
            .service(handlers::add_schedule_entry)
            .service(handlers::update_schedule_entry)
            .service(handlers::delete_schedule_entry)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
