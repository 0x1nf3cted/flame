use std::sync::{Arc, Mutex};

use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use bcrypt::BcryptError;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use thiserror::Error;

extern crate bcrypt;
use crate::utils::redis::set_data;
#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    email: String,
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("hashing error")]
    HashingError(#[from] BcryptError),
}

impl actix_web::ResponseError for CustomError {}

#[post("/login")]
pub async fn signup(
    body: Json<UserInfo>,
    pool: web::Data<Pool<Postgres>>,
    rds: web::Data<Arc<Mutex<redis::Connection>>>,
) -> impl Responder {
    let b = body.into_inner();
    let s = get_user(b.clone(), pool.clone()).await;

    match s {
        Ok(_) => {
            println!("signup successful");
            match set_data(&format!("confirm-{}", &b.email), rds).await {
                Ok(code) => {
                    return HttpResponse::Created()
                        .body(format!("user found and confirmation code set {}", &code))
                }
                Err(e) => {
                    eprintln!("error creating confirmation code: {}", e);
                    return HttpResponse::InternalServerError().body("internal server error");
                }
            }
        }
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                if let Err(e) = create_user(b.clone(), pool.clone()).await {
                    eprintln!("error creating user: {}", e);
                    return HttpResponse::InternalServerError().body("internal server error");
                }

                match set_data(&format!("confirm-{}", &b.email), rds).await {
                    Ok(code) => {
                        return HttpResponse::Created()
                            .body(format!("user created and confirmation code set {}", &code))
                    }
                    Err(e) => {
                        eprintln!("error creating confirmation code: {}", e);
                        return HttpResponse::InternalServerError().body("internal server error");
                    }
                }
            }
            _ => {
                eprintln!("internal server error: {:?}", e);
                return HttpResponse::InternalServerError().body("internal server error");
            }
        },
    }
}

pub async fn create_user(
    user_info: UserInfo,
    pool: web::Data<Pool<Postgres>>,
) -> Result<(), CustomError> {
    let query = "insert into accounts (id, email) values ($1, $2)";
    let id = Uuid::new_v4().to_string();
    sqlx::query(query)
        .bind(&id)
        .bind(&user_info.email)
        .execute(&**pool)
        .await?;

    Ok(())
}

pub async fn get_user(
    user_info: UserInfo,
    pool: web::Data<Pool<Postgres>>,
) -> Result<UserInfo, sqlx::Error> {
    let row = sqlx::query_as::<_, UserInfo>("select * from accounts where email = $1")
        .bind(user_info.email)
        .fetch_one(&**pool)
        .await?;

    Ok(row)
}
