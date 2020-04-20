use actix_web::{HttpResponse, web, Result};
use serde::{Deserialize};
use crate::models::{credentials::Credential, user::User};
use crate::db::Pool;
use super::{success};

#[derive(Deserialize)]
pub struct UserDTO {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct CredentialDTO {
    user: UserDTO,
    credential: Credential
}

pub async fn signup(user: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_result = User::create(&pool.get().unwrap(), user.0.username, user.0.password);
    if user_result.is_err() {
        return success(false);
    }
    Ok(HttpResponse::Ok().json(user_result.unwrap().credentials))
}

pub async fn login(user: web::Json<UserDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user_result = User::login(&pool.get().unwrap(), user.0.username, user.0.password);
    if user_result.is_err() {
        return success(false);
    }
    Ok(HttpResponse::Ok().json(user_result.unwrap().credentials))
}

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