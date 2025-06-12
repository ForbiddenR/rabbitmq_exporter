use std::collections::HashMap;

use prometheus::GaugeVec;
use serde_json::Value;

use crate::{
    config::Config, error::Error, exporter::{new_gauge_vec, parse_value, request}, set_field, set_gauge_vec
};

pub struct OverviewExporter {
    pub node_info: Option<NodeInfo>,
    pub version_metric: GaugeVec,
    pub metric_description: HashMap<String, GaugeVec>,
}

impl OverviewExporter {
    pub fn new() -> Self {
        let overview_lables = ["cluster"];
        let metric_description = HashMap::from([
            set_gauge_vec!(
                "object_totals.queues",
                "queues",
                "Number of queues in use.",
                &overview_lables
            ),
            set_gauge_vec!(
                "queue_totals.messages",
                "queue_messages_global",
                "Number ready and unacknowledged messages in cluster.",
                &overview_lables
            ),
            set_gauge_vec!(
                "queue_totals.messages_ready",
                "queue_messages_ready_global",
                "Number of messages ready to be delivered to clients.",
                &overview_lables
            ),
            set_gauge_vec!(
                "queue_totals.messages_unacknowledged",
                "queue_messages_unacknowledged_global",
                "Number of messages delivered to clients but not yet acknowledged.",
                &overview_lables
            ),
            set_gauge_vec!(
                "message_stats.publish_details.rate",
                "messages_publish_rate",
                "Rate at which messages are entering the server.",
                &overview_lables
            ),
            set_gauge_vec!(
                "message_stats.deliver_no_ack_details.rate",
                "messages_deliver_no_ack_rate",
                "Rate at which messages are delivered to consumers that use automatic acknowledgements.",
                &overview_lables
            ),
            set_gauge_vec!(
                "message_stats.deliver_details.rate",
                "messages_deliver_rate",
                "Rate at which messages are delivered to consumers that use manual acknowledgements.",
                &overview_lables
            ),
        ]);

        Self {
            node_info: None,
            metric_description,
            version_metric: new_gauge_vec(
                "rabbitmq_version_info",
                "A metric with a constant '1' value labeled by rabbitmq version, erlang version, node, cluster.",
                &["rabbitmq", "erlang", "node", "cluster"],
            ),
        }
    }

    pub fn get_cluster_name(&self) -> String {
        match &self.node_info {
            Some(t) => t.cluster_name.clone(),
            None => "".into(),
        }
    }

    pub fn clear_metrics(&self) {
        self.metric_description.iter().for_each(|(_, v)|v.reset());
        let node = self.node_info.clone().unwrap_or_default();
        self.version_metric.with_label_values(&[
            &node.rabbitmq_version,
            &node.erlang_version,
            &node.node,
            &node.cluster_name,
        ]).set(0.0);
    }

    pub async fn collect(&mut self, config: &Config) -> Result<(), Error> {
        self.metric_description.iter().for_each(|(_, v)| v.reset());
        let response = request(config, "overview").await?.json::<Value>().await?;

        let node = if self.node_info.is_none() {
            self.node_info = Some(NodeInfo::from_value(response.clone()));
            self.node_info.as_ref().unwrap()
        } else {
            self.node_info.as_ref().unwrap()
        };

        self.version_metric
            .with_label_values(&[
                &node.rabbitmq_version,
                &node.erlang_version,
                &node.node,
                &node.cluster_name,
            ])
            .set(1.0);

        let value_map = parse_value(response);
        value_map.iter().for_each(|(k, &v)| {
            self.metric_description.get(k).map(|f| {
                f.with_label_values(&[self.node_info.as_ref().unwrap().cluster_name.clone()])
                    .set(v);
            });
        });
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct NodeInfo {
    pub node: String,
    pub cluster_name: String,
    pub erlang_version: String,
    pub rabbitmq_version: String,
}

impl NodeInfo {
    pub fn from_value(v: Value) -> Self {
        let mut node_info = NodeInfo::default();

        set_field!(node_info, v, "node", node);
        set_field!(node_info, v, "cluster_name", cluster_name);
        set_field!(node_info, v, "erlang_version", erlang_version);
        set_field!(node_info, v, "rabbitmq_version", rabbitmq_version);
        node_info
    }
}
