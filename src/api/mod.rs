pub mod user_controller;

use actix_web::{HttpResponse, Result};
use serde::{Serialize};

/// Represents a boolean response for a REST operation denoting its success
#[derive(Serialize)]
pub struct SuccessDTO {
    /// If operation is successful, this is set
    success: bool,
}

/// An HTTP JSON response that returns the success or failure of a REST operation.
///
/// # Arguments
/// 
/// * `is_success` - If the operation was a success or failure
pub fn success(is_success: bool) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(SuccessDTO { success: is_success }))
}

/// Serves as the asynchronous function for success, to be used in actix_web
pub async fn success_async() -> Result<HttpResponse> {
    success(true)
}