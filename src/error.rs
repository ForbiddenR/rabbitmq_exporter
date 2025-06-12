use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Cient(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Cient(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cient(e) => {
                write!(f, "client error: {e}")
            }
        }
    }
}
