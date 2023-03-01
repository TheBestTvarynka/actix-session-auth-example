use actix_web::{App, HttpServer};
use async_mutex::Mutex;
use tracing_actix_web::TracingLogger;

use crate::{
    api::{health, sign_in, sign_up},
    auth_service::AuthService,
    db::in_memory::{InMemoryAuthRepo, InMemorySessionRepo},
};

pub struct AppData {
    pub auth_service: Mutex<AuthService<InMemoryAuthRepo, InMemorySessionRepo>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            auth_service: Mutex::new(AuthService::new(
                InMemoryAuthRepo::default(),
                InMemorySessionRepo::default(),
            )),
        }
    }
}

#[allow(deprecated)]
pub async fn start_app() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .data(AppData::new())
            .service(health)
            .service(sign_up)
            .service(sign_in)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
