use std::sync::Arc;
use thiserror::Error;

extern crate bcrypt;
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use bcrypt::{hash, BcryptError, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInfo {
    username: String,
    password: String,
    email: String,
}

#[post("/signup")]
pub async fn signup(body: Json<UserInfo>, pool: web::Data<Arc<Pool<Postgres>>>) -> impl Responder {
    let b = body.into_inner();
    if let Err(e) = create_user(b.clone(), pool.clone()).await {
        eprintln!("Error creating user: {}", e);
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }
    return HttpResponse::Created().body(b.username + " Created!");
}
#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Hashing error")]
    HashingError(#[from] BcryptError),
}

impl actix_web::ResponseError for CustomError {}
pub async fn create_user(
    user_info: UserInfo,
    pool: web::Data<Arc<Pool<Postgres>>>,
) -> Result<(), CustomError> {
    let query = "INSERT INTO accounts (id, username, email, password) VALUES ($1, $2 , $3, $4)";
    let id = Uuid::new_v4().to_string();
    let password = hash(&user_info.password, DEFAULT_COST)?;
    sqlx::query(query)
        .bind(&id)
        .bind(&user_info.username)
        .bind(&user_info.email)
        .bind(&password)
        .execute(&***pool)
        .await?;

    Ok(())
}
