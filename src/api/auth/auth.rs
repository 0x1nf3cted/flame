use actix_web::{get, web::Json};
use derive_more::Display;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    username: String,
    password: String,
    generated_phrase: String,
}

#[get("/signup")]
pub async fn signup(body: Json<UserInfo>) -> Json<String> {
    return Json(body.into_inner().username);
}
