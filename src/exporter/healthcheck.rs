use anyhow::Result;
use prometheus::proto::MetricFamily;

use crate::{config::Conf, query, response::ping::HealthcheckResponse};

const ENDPOINT: &str = "healthchecks/node";

pub struct HealthcheckExporter {}

impl HealthcheckExporter {
    pub async fn collect(&mut self, config: &Conf) -> Result<Vec<MetricFamily>> {
        let resp: HealthcheckResponse = query!(config);
        if 
    }
}
