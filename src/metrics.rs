use crate::{
    config::Config,
    exporter::{overview::OverviewExporter, queue::QueueExporter},
};

pub struct Metrics {
    overview: OverviewExporter,
    queue: Option<QueueExporter>,
    config: Config,
}

impl Metrics {
    pub fn new(config: Config) -> Self {
        Metrics {
            config: config.clone(),
            overview: OverviewExporter::new(),
            // queue: QueueExporter::new(),
            queue: if config.enabled_exporters.contains(&"queue".to_owned()) {
                Some(QueueExporter::new())
            } else {
                None
            },
        }
    }

    pub async fn collect(&mut self) {
        if let Err(e) = self.overview.collect(&self.config).await {
            log::error!("failed to fetch overview messages: {e}");
            self.overview.clear_metrics();
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
