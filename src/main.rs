mod entity;
mod pkg;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::{Builder, Target};
use std::{env, time::Duration};
use pkg::logger::format_logger;

use entity::todo;
use sea_orm::{ConnectOptions, Database, DbConn, EntityTrait};

async fn get_items(db: web::Data<DbConn>) -> impl Responder {
    match todo::Entity::find().all(db.get_ref()).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => {
            eprintln!("Error fetching items: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn connect_db() -> DbConn {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish database connection
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug)
        .set_schema_search_path("public");

    Database::connect(opt).await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env::set_var("RUST_LOG", "debug");
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.format(format_logger);
    builder.init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Init DB
    let db = connect_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .route("/items", web::get().to(get_items))
    })
    .workers(1)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
