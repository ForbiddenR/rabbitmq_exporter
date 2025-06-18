use std::collections::HashMap;

use prometheus::GaugeVec;
use serde_json::Value;

use crate::{
    config::Conf,
    error::Error,
    exporter::{RabbitJsonReply, RabbitReply, new_gauge_vec, request},
    get_value, set_gauge_vec,
};

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

    pub async fn collect(&self, config: &Conf) -> Result<(), Error> {
        self.clear();
        let response = request(config, "queues").await?.json::<Value>().await?;

        RabbitJsonReply::from_response(&response)
            .make_stats_info(&vec!["name"])
            .iter()
            .for_each(|f| {
                let qname = get_value!(f.0, "name");

                self.queue_gauge_vec.iter().for_each(|k| {
                    let key = k.0.clone();
                    f.1.get(&key)
                        .map(|&va| k.1.with_label_values(&[&qname]).set(va));
                });
            });
        Ok(())
    }
}
