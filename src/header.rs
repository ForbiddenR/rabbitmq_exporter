use std::{ops::Deref, str::FromStr};

use actix_web::{
    error::ParseError,
    http::header::{Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue},
};

use crate::config::Mode;

#[derive(Debug)]
pub struct ExporterKey(pub Option<Mode>);

impl Deref for ExporterKey {
    type Target = Option<Mode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<&'a str> for ExporterKey {
    fn into(self) -> &'a str {
        self.0.map(|f| f.into()).unwrap_or("")
    }
}

impl TryIntoHeaderValue for ExporterKey {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<HeaderValue, Self::Error> {
        HeaderValue::from_str(self.into())
    }
}

impl Header for ExporterKey {
    fn name() -> HeaderName {
        HeaderName::from_str("EXPORTER_MODE").unwrap()
    }

    fn parse<M>(msg: &M) -> Result<Self, ParseError>
    where
        M: actix_web::HttpMessage,
    {
        Ok(ExporterKey(
            msg.headers()
                .get(Self::name())
                .map(|f| f.try_into())
                .transpose()
                .map_err(|_| ParseError::Header)?,
        ))
    }
}
