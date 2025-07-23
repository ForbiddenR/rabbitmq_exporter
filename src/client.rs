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
    Ok(reqwest::Client::new()
        .get(format!("{}/{}/{}", config.addr, "api", endpoint))
        .basic_auth(
            config.username.to_string(),
            Some(config.password.to_string()),
        )
        .header("Accept", "application/json")
        .timeout(Duration::from_secs(config.timeout as u64))
        .send()
        .await?
        .json()
        .await?)
}
