use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use dotenvy::dotenv;
use rabbitmq_exporter::{config::Conf, metrics::Metrics, routes};
use tokio::sync::RwLock;

#[actix_web::test]
async fn main() -> std::io::Result<()> {
    assert_eq!(dotenv().is_ok(), true);
    let config = Conf::build().unwrap();
    let shared_gauge = Arc::new(RwLock::new(Metrics::new(config)));

    let gauge_clone = shared_gauge.clone();
    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(gauge_clone.clone()))
            .service(routes::metrics)
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
