use prometheus::{Gauge, register_gauge};

use crate::{
    config::Conf,
    exporter::{overview::OverviewExporter, queue::QueueExporter},
};

pub struct Metrics {
    overview: OverviewExporter,
    queue: QueueExporter,
    up_metric: Gauge,
    config: Conf,
}

impl Metrics {
    pub fn new(config: Conf) -> Self {
        Metrics {
            config: config.clone(),
            overview: OverviewExporter::new(),
            up_metric: register_gauge!(
                "node_status",
                "Was the last scrape of rabbitmq successful."
            )
            .expect("Could not create gauge"),
            queue: QueueExporter::new(),
        }
    }

    pub async fn collect(&mut self, header: &str) {
        if let Err(e) = self
            .overview
            .collect(
                &self.config,
                self.config
                    .enabled_exporters
                    .contains(&"overview".to_owned())
                    || header.contains("overview"),
            )
            .await
        {
            log::error!("failed to fetch overview messages: {e}");
            return self.up_metric.set(0.0);
        } else {
            self.up_metric.set(1.0);
        }

        if self.config.enabled_exporters.contains(&"queue".to_owned()) || header.contains("queue") {
            if let Err(e) = self.queue.collect(&self.config).await {
                log::error!("failed to fetch queue message: {e}");
            }
        } else {
            self.queue.clear();
        }
    }
}
