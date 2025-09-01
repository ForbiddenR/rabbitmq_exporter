use std::time::Duration;

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::config::Conf;

#[macro_export]
macro_rules! query {
    ($conf:expr) => {
        request($conf, ENDPOINT).await?
    };
}

pub async fn request<T: DeserializeOwned>(config: &Conf, endpoint: &str) -> Result<T> {
    let resp = reqwest::Client::new()
        .get(format!("{}/{}/{}", config.addr, "api", endpoint))
        .basic_auth(
            config.username.to_string(),
            Some(config.password.to_string()),
        )
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(config.timeout as u64))
        .send()
        .await?;

    let status = resp.status();
    let text = resp.text().await?;

    if !status.is_success() {
        Err(anyhow::anyhow!(
            "Request failed with status: {}\n Response text: {}",
            status,
            text
        ))
    } else {
        match serde_json::from_str(&text) {
            Ok(data) => Ok(data),
            Err(e) => Err(anyhow::anyhow!(
                "Failed to parse JSON: {}\nResponse text: {}",
                e,
                text
            )),
        }
    }
}
