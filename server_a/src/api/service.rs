use actix_web::{middleware::Logger, web::JsonConfig, App, HttpServer};
use gutils::env::ENV;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::LazyLock,
};
use utoipa::openapi::Info;
use utoipa_actix_web::AppExt;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use super::service_on_start::DB_TICKERS;
use super::{activity, ticker};

pub static ADDR: LazyLock<SocketAddr> = LazyLock::new(|| {
    format!("{}:{}", Ipv4Addr::LOCALHOST, ENV.server_a_port)
        .parse()
        .unwrap()
});

pub async fn api() -> std::io::Result<()> {
    log::info!("Starting server on http://{}", *ADDR);
    log::info!("Server started with: {} tickers elements", DB_TICKERS.len());

    HttpServer::new(move || {
        let (app, mut api) = App::new()
            .app_data(JsonConfig::default().limit(
                100 * 1024 * 1024, //change limit of 2MB to 100MB
            ))
            .into_utoipa_app()
            .map(|app| app.wrap(Logger::default()))
            .service(ticker::ticker)
            .service(activity::activity)
            .split_for_parts();

        api.info = Info::default();
        api.info.title = "Ticker APIs".to_owned();
        api.info.description = Some("APIs".to_owned());

        app.service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", api.clone()),
        )
        .service(Redoc::with_url("/redoc", api))
    })
    .bind(*ADDR)?
    .run()
    .await
}
