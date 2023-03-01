mod api;
mod app;
mod auth_service;
mod db;
mod logging;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logging::set_up_logging();

    app::start_app().await
}
