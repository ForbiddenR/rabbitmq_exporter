use std::sync::Arc;

use actix_web::{HttpResponse, Responder, get, web};
use chrono::Utc;
use prometheus::{Encoder, TextEncoder};
use tokio::sync::RwLock;

use crate::metrics::Metrics;

#[get("/metrics")]
pub async fn metrics(metric: web::Data<Arc<RwLock<Metrics>>>) -> impl Responder {
    let start = Utc::now();
    {
        metric.write().await.collect().await;
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
