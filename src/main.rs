use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use tokio::sync::RwLock;

use crate::{config::Config, metrics::Metrics};

mod config;
mod error;
mod exporter;
mod metrics;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_log();
    let config = Config::read("rabbitmq.toml").unwrap();
    let port = config.publish_port;
    let shared_gauge = Arc::new(RwLock::new(Metrics::new(config)));

    let gauge_clone = shared_gauge.clone();
    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(gauge_clone.clone()))
            .service(routes::metrics)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn set_log() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
