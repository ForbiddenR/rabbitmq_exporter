use prometheus::{GaugeVec, register_gauge_vec};

use crate::{
    config::Conf,
    exporter::{overview::OverviewExporter, queue::QueueExporter},
};

pub struct Metrics {
    overview: OverviewExporter,
    queue: Option<QueueExporter>,
    up_metric: GaugeVec,
    config: Conf,
}

impl Metrics {
    pub fn new(config: Conf) -> Self {
        Metrics {
            config: config.clone(),
            overview: OverviewExporter::new(
                config.enabled_exporters.contains(&"overview".to_owned()),
            ),
            up_metric: register_gauge_vec!(
                "node_status",
                "Was the last scrape of rabbitmq successful.",
                &["node"]
            )
            .expect("Could not create gauge"),
            queue: if config.enabled_exporters.contains(&"queue".to_owned()) {
                Some(QueueExporter::new())
            } else {
                None
            },
        }
    }

    pub async fn collect(&mut self) {
        self.up_metric.reset();

        if let Err(e) = self.overview.collect(&self.config).await {
            log::error!("failed to fetch overview messages: {e}");
            return self.up_metric
                .with_label_values(&[self.overview.get_node_name()])
                .set(0.0)
        } else {
            self.up_metric
                .with_label_values(&[self.overview.get_node_name()])
                .set(1.0);
        }

        if let Some(q) = &self.queue {
            if let Err(e) = q.collect(&self.config).await {
                log::error!("failed to fetch queue message: {e}");
            }
        }
    }
}
