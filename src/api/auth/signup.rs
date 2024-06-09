use std::{error::Error, sync::Arc};

use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
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
    println!("hello");
    let b = body.into_inner();
    println!("hello");
    if let Err(e) = create_user(b.clone(), pool.clone()).await {
        eprintln!("Error creating user: {}", e);
        return HttpResponse::InternalServerError().finish();
    }
    println!("hello");
    return HttpResponse::Accepted().finish();
}
pub async fn create_user(
    user_info: UserInfo,
    pool: web::Data<Arc<Pool<Postgres>>>,
) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO accounts (id, username, email, password) VALUES ($1, $2 , $3, $4)";
    let id = Uuid::new_v4().to_string();
    sqlx::query(query)
        .bind(&id)
        .bind(&user_info.username)
        .bind(&user_info.email)
        .bind(&user_info.password)
        .execute(&***pool)
        .await?;

    Ok(())
}
