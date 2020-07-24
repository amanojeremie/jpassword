use actix_web::{HttpResponse, web, Result};
use serde::{Deserialize};
use crate::models::{credentials::Credential, user::User};
use crate::db::Pool;
use super::{success};

/// Represents a User of the application as provided in a POST request
/// as a JSON object
#[derive(Deserialize)]
pub struct UserDTO {
    /// The username of the user
    username: String,
    /// The hashed password of the user
    password: String,
}

/// Represents a user and their associated stored credentials to be returned over HTTP
/// as a JSON object
#[derive(Deserialize)]
pub struct CredentialDTO {
    /// A sub object that contains the user's credentials for this application
    user: UserDTO,
    /// A vector of the user's associated credentials for other applications
    credential: Credential
}

/// An endpoint for the creation of a new user, returning an HTTP response
/// that contains the User's new empty list of Credentials
pub async fn signup(user: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_result = User::create(&pool.get().unwrap(), user.0.username, user.0.password);
    if user_result.is_err() {
        return success(false);
    }
    Ok(HttpResponse::Ok().json(user_result.unwrap().credentials))
}

/// An endpoint for fetching an existing user, returning an HTTP response
/// that contains the User's saved list of credentials
pub async fn login(user: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_result = User::login(&pool.get().unwrap(), user.0.username, user.0.password);
    if user_result.is_err() {
        return success(false);
    }
    Ok(HttpResponse::Ok().json(user_result.unwrap().credentials))
}

/// An endpoint for adding a new credentials to an existing  User, returning an HTTP response
/// that contains the User's new list of credentials
pub async fn create(credential: web::Json<CredentialDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_result = User::login(&pool.get().unwrap(), credential.0.user.username, credential.0.user.password);
    
    if user_result.is_err() {
        return success(false);
    }
    let mut user = user_result.unwrap();
    let credential = credential.0.credential;

    user.credentials.create(
        credential.name, 
        credential.url,
        credential.username,
        credential.password);

    match user.save(&pool.get().unwrap()) {
        Ok(()) => Ok(HttpResponse::Ok().json(user.credentials)),
        Err(()) => success(false)
    }
}

/// An endpoint for deleting a saved credential of an existing User, returning an HTTP response
/// that contains the User's new list of credentials
pub async fn delete(index: web::Path<u32>, 
    user: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {

    let user_result = User::login(&pool.get().unwrap(), user.0.username, user.0.password);

    if user_result.is_err() {
        return success(false);
    }
    let mut user = user_result.unwrap();

    match user.credentials.delete(index.into_inner() as usize) {
        Ok(()) => {
            match user.save(&pool.get().unwrap()) {
                Ok(()) => Ok(HttpResponse::Ok().json(user.credentials)),
                Err(()) => success(false)
            }
        },
        Err(()) => success(false)
    }
}

/// An endpoint for updating a saved credential of an existing User, returning an HTTP response
/// that contains the User's new list of credentials after the update
pub async fn update(index: web::Path<u32>, 
    credential: web::Json<CredentialDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {

    let user_result = User::login(&pool.get().unwrap(), credential.0.user.username, credential.0.user.password);

    if user_result.is_err() {
        return success(false);
    }
    let mut user = user_result.unwrap();

    match user.credentials.update(index.into_inner() as usize, credential.0.credential) {
        Ok(()) => {
            match user.save(&pool.get().unwrap()) {
                Ok(()) => Ok(HttpResponse::Ok().json(user.credentials)),
                Err(()) => success(false)
            }
        },
        Err(()) => success(false)
    }
}