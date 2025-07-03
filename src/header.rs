use std::{ops::Deref, str::FromStr};

use actix_web::http::header::{
    Header, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderValue,
};

#[derive(Debug)]
pub struct ExporterKey(String);

impl Deref for ExporterKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryIntoHeaderValue for ExporterKey {
    type Error = InvalidHeaderValue;

    fn try_into_value(self) -> Result<actix_web::http::header::HeaderValue, Self::Error> {
        HeaderValue::from_str(&self.0)
    }
}

impl Header for ExporterKey {
    fn name() -> actix_web::http::header::HeaderName {
        HeaderName::from_str("ENABLED_EXPORTERS").unwrap()
    }

    fn parse<M>(msg: &M) -> Result<Self, actix_web::error::ParseError>
    where
        M: actix_web::HttpMessage,
    {
        let header_value = msg.headers().get(Self::name());

        if header_value.is_none() {
            return Ok(ExporterKey("".into()));
        } else {
            let header_value = header_value
                .unwrap()
                .to_str()
                .map_err(|_| actix_web::error::ParseError::Header)?;
            Ok(ExporterKey(header_value.into()))
        }
    }
}
