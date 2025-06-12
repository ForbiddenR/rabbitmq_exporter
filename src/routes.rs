use std::sync::Arc;

use actix_web::{HttpResponse, Responder, get, web};
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::metrics::Metrics;

#[get("/metrics")]
pub async fn metrics(metric: web::Data<Arc<RwLock<Metrics>>>) -> impl Responder {
    {
        metric.write().await.collect().await;
    }
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
