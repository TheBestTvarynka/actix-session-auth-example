use std::str::FromStr;

use actix_web::{cookie::Cookie, get, post, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    api::{SignInRequest, SignUpRequest},
    app::AppData,
};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("server is ok.")
}

#[post("/sign-up")]
pub async fn sign_up(
    sign_up_request: web::Json<SignUpRequest>,
    app: web::Data<AppData>,
) -> impl Responder {
    match app.auth_service.lock().await.sign_up(sign_up_request.0) {
        Ok(id) => HttpResponse::Ok().body(id.to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/sign-in")]
pub async fn sign_in(
    sign_in_request: web::Json<SignInRequest>,
    app: web::Data<AppData>,
) -> impl Responder {
    match app.auth_service.lock().await.sign_in(sign_in_request.0) {
        Ok(id) => {
            let mut response = HttpResponse::Created();

            response.cookie(Cookie::new("SessionId", id.to_string()));

            response.body(())
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profile")]
pub async fn profile(app: web::Data<AppData>, req: HttpRequest) -> impl Responder {
    let session_cookie = match req
        .cookie("SessionId")
        .ok_or_else(|| HttpResponse::Unauthorized().body("Missing session cookie"))
    {
        Ok(cookie) => cookie,
        Err(err) => return err,
    };

    match app
        .auth_service
        .lock()
        .await
        .profile(&Uuid::from_str(session_cookie.value()).unwrap())
    {
        Ok(user_data) => HttpResponse::Ok().body(serde_json::to_string(&user_data).unwrap()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
