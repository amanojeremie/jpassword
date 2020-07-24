mod db;
mod config;
mod crypto;
mod api;
mod models;

use actix_web::{http, App, HttpServer};
use actix_cors::{Cors};
use db::create_db_then_pool;

/// Creates an HTTP server serving as a RESTful interface for
/// password management
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let mut path_to_db = std::env::current_exe().unwrap();
    path_to_db.pop();
    path_to_db.push("jpassword.db".to_string());
    let pool = create_db_then_pool(path_to_db.as_path());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new()
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600)
                .finish())
            .data(pool.clone())
            .configure(config::app::configure)
            
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}