use anyhow::{Result, anyhow};
use prometheus::proto::MetricFamily;

use crate::{client::request, config::Conf, query, response::healthcheck::HealthcheckResponse};

// const ENDPOINT: &str = "healthchecks/node";
const ENDPOINT: &str = "health/checks/local-alarms";

pub struct HealthcheckExporter {}

impl HealthcheckExporter {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn collect(&self, config: &Conf) -> Result<Vec<MetricFamily>> {
        let resp: HealthcheckResponse = query!(config);
        if resp.is_ok() {
            Ok(vec![])
        } else {
            Err(anyhow!(format!("abnornal resp status: {}", resp.status)))
        }
    }
}
