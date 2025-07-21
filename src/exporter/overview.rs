use std::collections::HashMap;

use anyhow::Result;
use prometheus::{GaugeVec, Opts, core::Collector, proto::MetricFamily};
use serde_json::Value;

use crate::{
    client::request,
    config::Conf,
    exporter::{RabbitJsonReply, RabbitReply},
    query, set_gauge, set_gauge_vec,
};

const ENDPOINT: &str = "overview";
const EMPTOY_LABEL: &[&str] = &[];

pub struct OverviewExporter {
    // node_info: Option<NodeInfo>,
    metric_description: HashMap<String, GaugeVec>,
}

impl OverviewExporter {
    pub fn new() -> Self {
        // let overview_lables = ["cluster"];
        let metric_description = HashMap::from([
            set_gauge!("object_totals.queues", "queues", "Number of queues in use."),
            set_gauge!(
                "queue_totals.messages",
                "queue_messages_global",
                "Number ready and unacknowledged messages in cluster."
            ),
            set_gauge!(
                "queue_totals.messages_ready",
                "queue_messages_ready_global",
                "Number of messages ready to be delivered to clients."
            ),
            set_gauge!(
                "queue_totals.messages_unacknowledged",
                "queue_messages_unacknowledged_global",
                "Number of messages delivered to clients but not yet acknowledged."
            ),
            set_gauge!(
                "message_stats.publish_details.rate",
                "messages_publish_rate",
                "Rate at which messages are entering the server."
            ),
            set_gauge!(
                "message_stats.deliver_no_ack_details.rate",
                "messages_deliver_no_ack_rate",
                "Rate at which messages are delivered to consumers that use automatic acknowledgements."
            ),
            set_gauge!(
                "message_stats.deliver_details.rate",
                "messages_deliver_rate",
                "Rate at which messages are delivered to consumers that use manual acknowledgements."
            ),
        ]);

        Self { metric_description }
    }

    pub fn clear(&self) {
        self.metric_description.iter().for_each(|(.., f)| f.reset());
    }

    pub async fn collect(&mut self, config: &Conf) -> Result<Vec<MetricFamily>> {
        self.clear();
        let resp: Value = query!(config);

        RabbitJsonReply::from_response(&resp)
            .make_map()
            .iter()
            .for_each(|(k, &v)| {
                self.metric_description
                    .get(k)
                    .map(|f| f.with_label_values(EMPTOY_LABEL).add(v));
            });
        Ok(self
            .metric_description
            .values()
            .flat_map(|f| f.collect())
            .collect())
    }
}
