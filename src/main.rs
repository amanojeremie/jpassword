mod db;
mod config;
mod crypto;
mod api;
mod models;

use actix_web::{http, App, HttpServer};
use actix_cors::{Cors};
use db::create_db_then_pool;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let pool = create_db_then_pool("./test.db".to_string());

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