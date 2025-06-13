use std::collections::HashMap;

use prometheus::GaugeVec;
use serde_json::Value;

use crate::{
    config::Conf,
    error::Error,
    exporter::{make_status_info, new_gauge_vec, request},
    get_value, set_gauge_vec,
};

#[derive(Clone)]
pub struct QueueExporter {
    pub cluster_name: String,
    pub queue_gauge_vec: HashMap<String, GaugeVec>,
}

impl QueueExporter {
    pub fn new() -> Self {
        let queue_labels = ["queue"];
        let queue_gauge_vec = HashMap::from([
            // set_gauge_vec!(
            //     "messages_ready",
            //     "queue_messages_ready",
            //     "Number of messages ready to be delivered to clients.",
            //     &queue_labels
            // ),
            // set_gauge_vec!(
            //     "messages_unacknowledged",
            //     "queue_messages_unacknowledged",
            //     "Number of messages delivered to clients but not yet acknowledged.",
            //     &queue_labels
            // ),
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

    pub async fn collect(&self, config: &Conf) -> Result<(), Error> {
        self.queue_gauge_vec.iter().for_each(|(_, f)| f.reset());
        let response = request(config, "queues").await?.json::<Value>().await?;

        let result = make_status_info(
            response,
            &vec![
                // "vhost",
                "name",
                //  "durable",
                // "policy",
                // "state",
                // "node",
                // "idle_since",
            ],
        );

        result.iter().for_each(|f| {
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
