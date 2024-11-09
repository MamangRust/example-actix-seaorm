use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use example_crud_blog_seaorm::{handler::router_config, state};
use sea_orm::{Database, DatabaseConnection};
use example_crud_blog_seaorm::utils::tracing;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    tracing();


    let app_state = state::AppState::new(db.clone());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .configure(router_config)
            .app_data(Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    .expect("Failed");

    Ok(())
}
