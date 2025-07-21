use anyhow::Result;
use prometheus::{Gauge, core::Collector, proto::MetricFamily, register_gauge};

use crate::{
    config::Conf,
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
            up_metric: register_gauge!(
                "node_status",
                "Was the last scrape of rabbitmq successful."
            )
            .expect("Could not create gauge"),
            queue: QueueExporter::new(),
        }
    }

    fn is_exported(&self, exporter: &str) -> bool {
        self.config.enabled_exporters.contains(&exporter.to_owned())
    }

    async fn ping(&self) -> Result<Vec<MetricFamily>> {
        self.healthcheck.collect(&self.config).await
    }

    async fn opt(&self, header: &str) -> Result<Vec<MetricFamily>> {
        macro_rules! collect {
            ($p:expr $(, $tag:literal, $m:ident)+) => {{
                $(
                    if self.is_exported($tag) || header.contains($tag) {
                        $p.extend(self.$m.collect(&self.config).await?);
                    }
                )+
            }};
        }
        let mut result = vec![];
        collect!(&mut result, "overview", overview, "queue", queue);
        Ok(result)
    }

    pub async fn collect(&self, header: &str) -> Vec<MetricFamily> {
        match if header.is_empty() && self.config.enabled_exporters.is_empty() {
            self.ping().await
        } else {
            self.opt(header).await
        } {
            Ok(m) => {
                self.up_metric.set(1.0);
                m
            }
            Err(e) => {
                log::error!("failed to collect metrics: {e}");
                self.up_metric.set(0.0);
                vec![]
            }
        }
        .into_iter()
        .chain(self.up_metric.collect())
        .filter(|f| !f.get_metric().is_empty())
        .collect()
    }
}
