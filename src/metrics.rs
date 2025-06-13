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
            overview: OverviewExporter::new(),
            up_metric: register_gauge_vec!(
                "node_status",
                "Was the last scrape of rabbitmq successful.",
                &["node"]
            )
            .expect("Could not create gauge"),
            // queue: QueueExporter::new(),
            queue: if config.enabled_exporters.is_some()
                && config
                    .enabled_exporters
                    .unwrap()
                    .contains(&"queue".to_owned())
            {
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
            self.up_metric
                .with_label_values(&[
                    self.overview.get_node_name(),
                ])
                .set(0.0);
        } else {
            self.up_metric
                .with_label_values(&[
                    self.overview.get_node_name(),
                ])
                .set(1.0);
        }

        if let Some(q) = self.queue.as_mut() {
            if let Err(e) = q
                .set_cluster_name(self.overview.get_cluster_name())
                .collect(&self.config)
                .await
            {
                log::error!("failed to fetch queue message: {e}");
            }
        }
    }
}
