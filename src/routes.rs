use std::{sync::Arc, vec};

use actix_web::{HttpRequest, HttpResponse, Responder, get, web};
use chrono::Utc;
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::metrics::Metrics;

#[get("/metrics")]
pub async fn metrics(req: HttpRequest, metric: web::Data<Arc<RwLock<Metrics>>>) -> impl Responder {
    let value = req
        .headers()
        .get("ENABLED_EXPORTERS")
        .map(|f| f.to_str().unwrap_or_default())
        .unwrap_or_default();

    let start = Utc::now();
    {
        metric.write().await.collect(value).await;
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
