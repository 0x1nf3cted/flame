use actix_web::web;
use redis::{Commands, RedisError};
use std::sync::{Arc, Mutex};

use super::helpers::generate_random_6_digits;

pub async fn set_data(
    data: &str,
    rds: web::Data<Arc<Mutex<redis::Connection>>>,
) -> Result<String, RedisError> {
    let confirmation_code = generate_random_6_digits().to_string();

    let mut conn = rds.lock().unwrap();

    conn.set(&data, &confirmation_code)?;
    conn.expire(&data, 300)?;

    Ok(confirmation_code)
}
pub async fn get_data(
    label: &str,
    rds: web::Data<Arc<Mutex<redis::Connection>>>,
) -> Result<String, RedisError> {
    let mut conn = rds.lock().unwrap();

    let data: String = conn.get(&label)?;

    Ok(data)
}
pub async fn del_data(
    label: &str,
    rds: web::Data<Arc<Mutex<redis::Connection>>>,
) -> Result<(), RedisError> {
    let mut conn = rds.lock().unwrap();

    let _ = conn.del(&label)?;

    Ok(())
}
