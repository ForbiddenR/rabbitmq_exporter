use std::collections::HashMap;

use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};
use serde_json::Value;

use crate::{
    client::request,
    config::Conf,
    exporter::{RabbitJsonReply, RabbitReply},
    get_value, query, set_gauge_vec,
};

const ENDPOINT: &str = "queues";

#[derive(Clone)]
pub struct QueueExporter {
    queue_gauge_vec: HashMap<String, GaugeVec>,
}

impl QueueExporter {
    pub fn new() -> Self {
        let queue_labels = ["queue"];
        let queue_gauge_vec = HashMap::from([set_gauge_vec!(
            "messages",
            "queue_messages",
            "Sum of ready and unacknowledged messages (queue depth).",
            &queue_labels
        )]);
        QueueExporter { queue_gauge_vec }
    }

    pub fn clear(&self) {
        self.queue_gauge_vec.iter().for_each(|(_, f)| f.reset());
    }

    pub async fn collect(&self, config: &Conf) -> Result<Vec<MetricFamily>> {
        self.clear();
        let resp: Value = query!(config);

        RabbitJsonReply::from_response(&resp)
            .make_stats_info(&vec!["name"])
            .iter()
            .for_each(|f| {
                let qname = get_value!(f.0, "name");

                self.queue_gauge_vec.iter().for_each(|k| {
                    f.1.get(k.0.as_str())
                        .map(|&va| k.1.with_label_values(&[&qname]).set(va));
                });
            });

        Ok(self
            .queue_gauge_vec
            .values()
            .flat_map(|f| f.collect())
            .collect())
    }
}
