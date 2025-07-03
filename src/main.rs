use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use rabbitmq_exporter::{config::Conf, metrics::Metrics, routes, set_log};
use tokio::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_log();
    let config = Conf::build().unwrap();
    let shared_gauge = Arc::new(RwLock::new(Metrics::new(config)));

    let gauge_clone = shared_gauge.clone();
    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(gauge_clone.clone()))
            .service(routes::metrics)
            .service(routes::heartbeat)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
