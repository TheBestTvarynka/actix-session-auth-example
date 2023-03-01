use std::sync::Arc;

use actix_web::{App, HttpServer};
use async_mutex::Mutex;
use tracing_actix_web::TracingLogger;

use crate::{
    api::{health, profile, sign_in, sign_up},
    auth_service::AuthService,
    db::in_memory::{InMemoryAuthRepo, InMemorySessionRepo},
};

pub struct AppData {
    pub auth_service: Arc<Mutex<AuthService<InMemoryAuthRepo, InMemorySessionRepo>>>,
}

impl AppData {
    pub fn new(
        auth_service: Arc<Mutex<AuthService<InMemoryAuthRepo, InMemorySessionRepo>>>,
    ) -> Self {
        tracing::debug!("Creating AppData...");

        Self { auth_service }
    }
}

#[allow(deprecated)]
pub async fn start_app() -> std::io::Result<()> {
    let auth_service = Arc::new(Mutex::new(AuthService::new(
        InMemoryAuthRepo::default(),
        InMemorySessionRepo::default(),
    )));

    HttpServer::new(move || {
        tracing::debug!("Creating HTTP server...");

        App::new()
            .wrap(TracingLogger::default())
            .data(AppData::new(auth_service.clone()))
            .service(health)
            .service(sign_up)
            .service(sign_in)
            .service(profile)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
