use std::fmt::{Display, Formatter};

use crate::error::ShodanError::*;

#[derive(Debug)]
pub enum ShodanError {
    ShodanClientError(String),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for ShodanError {
    fn from(e: reqwest::Error) -> Self {
        ReqwestError(e)
    }
}

impl Display for ShodanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShodanClientError(err) => write!(f, "Shodan client error: {err}"),
            ReqwestError(err) => write!(f, "Reqwest error: {err}"),
        }
    }
}
