use actix_web::{cookie::Cookie, delete, HttpResponse, Responder};

#[delete("/logout")]
pub async fn logout() -> impl Responder {
    let session_name = std::env::var("SESSION_NAME").expect("SESSION_NAME must be set");
    let mut c = Cookie::build(session_name, "token")
        .domain("localhost")
        .path("/")
        .http_only(true)
        .finish();

    c.make_removal();
    return HttpResponse::Ok().cookie(c).body("Logged out successfully");
}
