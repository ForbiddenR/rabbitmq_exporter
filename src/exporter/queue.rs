use std::collections::HashMap;

use prometheus::GaugeVec;
use serde_json::Value;

use crate::{
    config::Config, error::Error, exporter::{make_status_info, new_gauge_vec, request}, get_value, set_gauge_vec
};

#[derive(Clone)]
pub struct QueueExporter {
    pub cluster_name: String,
    pub queue_gauge_vec: HashMap<String, GaugeVec>,
}

impl QueueExporter {
    pub fn new() -> Self {
        let queue_labels = ["cluster", "vhost", "queue", "durable", "policy"];
        let queue_gauge_vec = HashMap::from([
            set_gauge_vec!(
                "messages_ready",
                "queue_messages_ready",
                "Number of messages ready to be delivered to clients.",
                &queue_labels
            ),
            set_gauge_vec!(
                "messages_unacknowledged",
                "queue_messages_unacknowledged",
                "Number of messages delivered to clients but not yet acknowledged.",
                &queue_labels
            ),
            set_gauge_vec!(
                "messages",
                "queue_messages",
                "Sum of ready and unacknowledged messages (queue depth).",
                &queue_labels
            ),
        ]);
        QueueExporter {
            cluster_name: "".into(),
            queue_gauge_vec,
        }
    }

    pub fn set_cluster_name(&mut self, cluster_name: String) -> &Self {
        self.cluster_name = cluster_name;
        self
    }

    pub async fn collect(&self, config: &Config) -> Result<(), Error> {
        self.queue_gauge_vec.iter().for_each(|(_, f)| f.reset());
        let response = request(config, "queues").await?.json::<Value>().await?;

        let result = make_status_info(
            response,
            &vec![
                "vhost", "name", "durable",
                "policy",
                // "state",
                // "node",
                // "idle_since",
            ],
        );

        result.iter().for_each(|f| {
            let qname = get_value!(f.0, "name");
            let vname = get_value!(f.0, "vhost");
            let durable = get_value!(f.0, "durable");
            let policy = get_value!(f.0, "policy");
            let label_values = [&self.cluster_name, &vname, &qname, &durable, &policy];

            self.queue_gauge_vec.iter().for_each(|k| {
                let key = k.0.clone();
                f.1.get(&key)
                    .map(|&va| k.1.with_label_values(&label_values).set(va));
            });
        });
        Ok(())
    }
}
