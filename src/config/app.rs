use actix_web::web;
use crate::api::{self, user_controller};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/jpassword")
            .service(web::resource("/").route(web::get().to(api::success_async)))
            .service(web::resource("/signup").route(web::post().to(user_controller::signup)))
            .service(web::resource("/login").route(web::post().to(user_controller::login)))
            .service(web::resource("/credential").route(web::post().to(user_controller::create)))
            .service(web::resource("/credential/{id}")
                .route(web::delete().to(user_controller::delete))
                .route(web::put().to(user_controller::update)))
    );
}