use std::{sync::Arc, vec};

use actix_web::{HttpResponse, Responder, dev::Response, get, web};
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::{header::ExporterKey, metrics::Metrics};

#[get("/heartbeat")]
pub async fn heartbeat() -> impl Responder {
    Response::ok()
}

#[get("/metrics")]
pub async fn metrics(
    header: web::Header<ExporterKey>,
    exporter: web::Data<Arc<RwLock<Metrics>>>,
) -> impl Responder {
    // let start = Utc::now();
    let metric;
    {
        metric = exporter.write().await.collect(&header).await;
    }
    // let duration = Utc::now() - start;
    // log::info!("wait {} milliseconds", duration.num_milliseconds());

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    match encoder.encode(&metric, &mut buffer) {
        Err(e) => HttpResponse::BadGateway().body(format!("{e}")),
        _ => HttpResponse::Ok().body(buffer),
    }
}
