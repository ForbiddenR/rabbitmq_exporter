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
    // pub version_metric: GaugeVec,
    metric_description: HashMap<String, Gauge>,
}

impl OverviewExporter {
    pub fn new(enabled: bool) -> Self {
        // let overview_lables = ["cluster"];
        let metric_description = if enabled {
            HashMap::from([
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
            ])
        } else {
            HashMap::new()
        };

        Self {
            node_info: None,
            metric_description,
            // version_metric: new_gauge_vec(
            //     "rabbitmq_version_info",
            //     "A metric with a constant '1' value labeled by rabbitmq version, erlang version, node, cluster.",
            //     &["rabbitmq", "erlang", "node", "cluster"],
            // ),
        }
    }

    // pub fn get_cluster_name(&self) -> String {
    //     match &self.node_info {
    //         Some(t) => t.cluster_name.clone(),
    //         None => "".into(),
    //     }
    // }

    pub fn get_node_name(&self) -> String {
        match &self.node_info {
            Some(t) => t.node.clone(),
            None => String::from(""),
        }
    }

    pub async fn collect(&mut self, config: &Conf) -> Result<(), Error> {
        // self.metric_description.iter().for_each(|(_, v)| v.reset());
        // self.version_metric.reset();
        let response = request(config, "overview").await?.json::<Value>().await?;

        // let node = if self.node_info.is_none() {
        //     self.node_info = Some(NodeInfo::from_value(response.clone()));
        //     self.node_info.as_ref().unwrap()
        // } else {
        //     self.node_info
        //         .as_mut()
        //         .unwrap()
        //         .update_cluster_and_node(response.clone());
        //     self.node_info.as_ref().unwrap()
        // };

        if self.node_info.is_none() {
            self.node_info = Some(NodeInfo::from_value(response.clone()));
        } else {
            self.node_info
                .as_mut()
                .unwrap()
                .update_node(response.clone());
        }

        if !self.metric_description.is_empty() {
            let value_map = parse_value(response);
            value_map.iter().for_each(|(k, &v)| {
                self.metric_description.get(k).map(|f| f.set(v));
            });
        }

        // self.version_metric
        //     .with_label_values(&[
        //         &node.rabbitmq_version,
        //         &node.erlang_version,
        //         &node.node,
        //         &node.cluster_name,
        //     ])
        //     .set(1.0);

        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NodeInfo {
    pub node: String,
    // pub cluster_name: String,
    // pub erlang_version: String,
    // pub rabbitmq_version: String,
}

impl NodeInfo {
    pub fn from_value(v: Value) -> Self {
        let mut node_info = NodeInfo::default();

        set_field!(node_info, v, "node", node);
        // set_field!(node_info, v, "cluster_name", cluster_name);
        // set_field!(node_info, v, "erlang_version", erlang_version);
        // set_field!(node_info, v, "rabbitmq_version", rabbitmq_version);
        node_info
    }

    // pub fn update_cluster_and_node(&mut self, v: Value) {
    //     set_field!(self, v, "node", node);
    //     set_field!(self, v, "cluster_name", cluster_name);
    // }

    pub fn update_node(&mut self, v: Value) {
        set_field!(self, v, "node", node);
    }
}
