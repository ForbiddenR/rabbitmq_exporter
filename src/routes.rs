use std::{sync::Arc, vec};

use actix_web::{HttpResponse, Responder, get, web};
use chrono::Utc;
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::{header::ExporterKey, metrics::Metrics};

#[get("/metrics")]
pub async fn metrics(
    header: web::Header<ExporterKey>,
    metric: web::Data<Arc<RwLock<Metrics>>>,
) -> impl Responder {
    let start = Utc::now();
    {
        metric.write().await.collect(&header.to_string()).await;
    }
    let duration = Utc::now() - start;
    log::info!("wait {} milliseconds", duration.num_milliseconds());

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let result = encoder.encode(&metric_families, &mut buffer);
    if let Ok(_) = result {
        HttpResponse::Ok().body(buffer)
    } else {
        HttpResponse::BadGateway().body("Failed to get prometheus messages")
    }
}
