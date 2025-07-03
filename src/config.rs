use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Conf {
    pub rabbit_url: String,
    pub rabbit_user: String,
    pub rabbit_pass: String,
    #[serde(default)]
    pub enabled_exporters: Vec<String>,
    pub timeout: u32,
}

impl Conf {
    pub fn build() -> Result<Conf, Error> {
        Ok(envy::from_env::<Conf>()?)
    }
}

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use crate::config::Conf;

    #[test]
    fn read_config() {
        dotenv().unwrap();

        let config = Conf::build().unwrap();

        println!("{:?}", config);
    }
}
