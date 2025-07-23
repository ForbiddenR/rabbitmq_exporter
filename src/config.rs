use std::str::FromStr;

use serde::Deserialize;

use anyhow::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Conf {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub exporter_mode: Mode,
    #[serde(default = "Conf::default_timeout")]
    pub timeout: u32,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Mode {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "simple")]
    Simple,
}

impl<'a> Into<&'a str> for Mode {
    fn into(self) -> &'a str {
        match self {
            Mode::Standard => "standard",
            Mode::Simple => "simple",
        }
    }
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Mode::Standard),
            "simple" => Ok(Mode::Simple),
            _ => Err("Invalid mode, must be 'standard' or 'simple'"),
        }
    }
}

impl TryFrom<&actix_web::http::header::HeaderValue> for Mode {
    type Error = &'static str;

    fn try_from(
        v: &actix_web::http::header::HeaderValue,
    ) -> std::result::Result<Self, Self::Error> {
        v.to_str().map_err(|_| "Invalid header value")?.parse()
    }
}

impl Conf {
    pub fn build() -> Result<Conf> {
        Ok(envy::from_env::<Conf>()?)
    }

    fn default_timeout() -> u32 {
        1
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
