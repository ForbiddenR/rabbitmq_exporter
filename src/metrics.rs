use anyhow::Result;
use prometheus::{Gauge, core::Collector, proto::MetricFamily};

use crate::{
    config::{Conf, Mode},
    exporter::{
        healthcheck::HealthcheckExporter, overview::OverviewExporter, queue::QueueExporter,
    },
};

pub struct Metrics {
    overview: OverviewExporter,
    queue: QueueExporter,
    healthcheck: HealthcheckExporter,
    up_metric: Gauge,
    config: Conf,
}

impl Metrics {
    pub fn new(config: Conf) -> Self {
        Metrics {
            config: config.clone(),
            overview: OverviewExporter::new(),
            healthcheck: HealthcheckExporter::new(),
            up_metric: Gauge::new("node_status", "Was the last scrape of rabbitmq successful.")
                .expect("Could not create gauge"),
            queue: QueueExporter::new(),
        }
    }

    async fn ping(&self) -> Result<Vec<MetricFamily>> {
        self.healthcheck.collect(&self.config).await
    }

    async fn all(&self) -> Result<Vec<MetricFamily>> {
        Ok(self
            .overview
            .collect(&self.config)
            .await?
            .into_iter()
            .chain(self.queue.collect(&self.config).await?)
            .filter(|f| !f.get_metric().is_empty())
            .collect())
    }

    pub async fn collect(&self, mode: &Option<Mode>) -> Vec<MetricFamily> {
        match match mode.as_ref().unwrap_or(&self.config.exporter_mode) {
            Mode::Standard => self.all().await,
            Mode::Simple => self.ping().await,
        } {
            Ok(m) => {
                self.up_metric.set(1.0);
                m
            }
            Err(e) => {
                log::error!("failed to collect metrics: {e}");
                self.up_metric.set(0.0);
                return self.up_metric.collect();
            }
        }
        .into_iter()
        .chain(self.up_metric.collect())
        .filter(|f| !f.get_metric().is_empty())
        .collect()
    }
}
