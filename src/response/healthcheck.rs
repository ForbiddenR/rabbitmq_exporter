use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HealthcheckResponse {
    pub status: String,
}

impl HealthcheckResponse {
    pub fn is_ok(&self) -> bool {
        &self.status == "ok"
    }
}
