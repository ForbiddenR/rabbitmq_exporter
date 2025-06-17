use std::collections::HashMap;

use prometheus::Gauge;
use serde_json::Value;

use crate::{
    config::Conf,
    error::Error,
    exporter::{new_gauge, parse_value, request},
    set_field, set_gauge,
};

pub struct OverviewExporter {
    node_info: Option<NodeInfo>,
    metric_description: HashMap<String, Gauge>,
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

        Self {
            node_info: None,
            metric_description,
        }
    }

    pub fn get_node_name(&self) -> String {
        match &self.node_info {
            Some(t) => t.node.clone(),
            None => String::from(""),
        }
    }

    pub fn clear(&self) {
        self.metric_description.iter().for_each(|(_, f)| f.set(0.0));
    }

    pub async fn collect(&mut self, config: &Conf, enable: bool) -> Result<(), Error> {
        self.clear();

        let response = request(config, "overview").await?.json::<Value>().await?;

        if self.node_info.is_none() {
            self.node_info = Some(NodeInfo::from_value(response.clone()));
        } else {
            self.node_info
                .as_mut()
                .unwrap()
                .update_node(response.clone());
        }

        if enable {
            parse_value(response).iter().for_each(|(k, &v)| {
                self.metric_description.get(k).map(|f| f.set(v));
            });
        }

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NodeInfo {
    pub node: String,
    pub test: String,
}

impl NodeInfo {
    pub fn from_value(v: Value) -> Self {
        let mut node_info = NodeInfo::default();

        set_field!(node_info, v, "node", node);
        node_info
    }

    pub fn update_node(&mut self, v: Value) {
        set_field!(self, v, "node", node);
    }
}
