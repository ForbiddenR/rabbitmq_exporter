use prometheus::{Gauge, core::Collector, proto::MetricFamily, register_gauge};

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

    fn is_exported(&self, exporter: &str) -> bool {
        self.config.enabled_exporters.contains(&exporter.to_owned())
    }

    pub async fn collect(&mut self, header: &str) -> Vec<MetricFamily> {
        macro_rules! collect {
            ($p:expr, $m:ident) => {
                $p.extend(match self.$m.collect(&self.config).await {
                    Ok(m) => {
                        self.up_metric.set(1.0);
                        m
                    }
                    Err(e) => {
                        log::error!("failed to fetch messages: {e}");
                        self.up_metric.set(0.0);
                        vec![]
                    }
                });
            };
        }

        let mut result = vec![];

        if self.is_exported("overview") || header.contains("overview") {
            collect!(&mut result, overview);
        }
        if self.is_exported("queue") || header.contains("queue") {
            collect!(&mut result, queue);
        }
        result
            .into_iter()
            .chain(self.up_metric.collect())
            .filter(|f| !f.get_metric().is_empty())
            .collect()

        // if let Err(e) = self
        //     .overview
        //     .collect(
        //         &self.config,
        //         self.is_exported("overview") || header.contains("overview"),
        //     )
        //     .await
        // {
        //     log::error!("failed to fetch overview messages: {e}");
        //     return self.up_metric.set(0.0);
        // } else {
        //     self.up_metric.set(1.0);
        // }

        // if self.is_exported("queue") || header.contains("queue") {
        //     if let Err(e) = self.queue.collect(&self.config).await {
        //         log::error!("failed to fetch queue message: {e}");
        //     }
        // } else {
        //     self.queue.clear();
        // }
    }
}
