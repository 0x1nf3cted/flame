use actix_web::{
    cookie::Cookie,
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use bcrypt::BcryptError;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::{Arc, Mutex};

use thiserror::Error;

extern crate bcrypt;
use crate::utils::redis::{del_data, get_data};

#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
pub struct ConfirmationBody {
    confirmation_code: String,
    email: String,
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Hashing error")]
    HashingError(#[from] BcryptError),
}

impl actix_web::ResponseError for CustomError {}
#[derive(sqlx::FromRow, Deserialize, Serialize, Clone)]
struct Claims {
    id: String,
    email: String,
    confirmed_email: bool,
    completed_account: bool,
}
#[post("/confirm")]
pub async fn confirmation(
    body: Json<ConfirmationBody>,
    pool: web::Data<Pool<Postgres>>,
    rds: web::Data<Arc<Mutex<redis::Connection>>>,
) -> impl Responder {
    let b = body.into_inner();
    let s = get_user(b.clone().email, pool.clone()).await;

    match s {
        Ok(user_data) => match get_data(&format!("confirm-{}", &b.email), rds.clone()).await {
            Ok(code) => {
                if code != b.confirmation_code {
                    println!("Code missmatch");
                    return HttpResponse::BadRequest().body("Invalid confirmation code");
                }
                if let Err(value) = confirm_email(b.clone(), pool).await {
                    println!("Error confirming email: {}", value);
                    return HttpResponse::InternalServerError().body("Internal Server Error");
                } else {
                    println!("email confirmed");
                };

                let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

                let claims = Claims {
                    id: user_data.id,
                    email: user_data.email,
                    confirmed_email: true,
                    completed_account: user_data.completed_account,
                };
                let token = match encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_ref()),
                ) {
                    Ok(t) => t,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };
                let _ = del_data(&format!("confirm-{}", &b.email), rds);

                let session_name = std::env::var("SESSION_NAME").expect("SESSION_NAME must be set");

                let c = Cookie::build(session_name, token)
                    .domain("localhost")
                    .path("/")
                    .http_only(true)
                    .finish();

                HttpResponse::Ok().body("Authenticated!");
                return HttpResponse::Ok().cookie(c).finish();
            }
            Err(e) => {
                eprintln!("Error getting confirmation code: {}", e);
                return HttpResponse::InternalServerError().body("Internal Server Error");
            }
        },
        Err(e) => match e {
            sqlx::Error::RowNotFound => {
                eprintln!("Error authenticating user: {}", e);
                return HttpResponse::BadRequest().body("Error authenticating user");
            }
            _ => {
                eprintln!("Internal Server Error: {:?}", e);
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        },
    }
}

pub async fn confirm_email(
    user_info: ConfirmationBody,
    pool: web::Data<Pool<Postgres>>,
) -> Result<(), CustomError> {
    let query = "UPDATE accounts SET confirmed_email = TRUE WHERE email = $1";
    sqlx::query(query)
        .bind(&user_info.email)
        .execute(&**pool)
        .await?;

    Ok(())
}

async fn get_user(
    user_info: String,
    pool: web::Data<Pool<Postgres>>,
) -> Result<Claims, sqlx::Error> {
    let row = sqlx::query_as::<_, Claims>("SELECT * FROM accounts WHERE email = $1")
        .bind(user_info)
        .fetch_one(&**pool)
        .await?;

    Ok(row)
}
