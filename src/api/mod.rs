pub mod user_controller;

use actix_web::{HttpResponse, Result};
use serde::{Serialize};

#[derive(Serialize)]
pub struct SuccessDTO {
    success: bool,
}


pub fn success(is_success: bool) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(SuccessDTO { success: is_success }))
}

pub async fn success_async() -> Result<HttpResponse> {
    success(true)
}