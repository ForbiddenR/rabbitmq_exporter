use std::{
    fmt::Display,
};

#[derive(Debug)]
pub enum Error {
    Cient(reqwest::Error),
    Config(envy::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Cient(err)
    }
}

impl From<envy::Error> for Error {
    fn from(err: envy::Error) -> Self {
        Self::Config(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cient(e) => write!(f, "client error: {e}"),
            Self::Config(e) => write!(f, "config error: {e}"),
        }
    }
}
