use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use crate::error::Error;

#[derive(Deserialize)]
pub(crate) struct HttpRequest {
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    pub(crate) fn header(&self, key: &str) -> Result<&String, Error> {
        self.headers
            .get(key)
            .ok_or_else(|| Error::HeaderNotFound(key.to_string()))
    }
}

#[derive(Serialize)]
pub(crate) struct HttpResponse {
    pub status: HttpStatus,
    pub body: String,
}

#[derive(Debug, Copy, Clone, Serialize_repr)]
#[repr(u16)]
pub(crate) enum HttpStatus {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    InternalServerError = 500,
}

#[derive(Debug)]
pub(crate) struct HttpError {
    pub(crate) status: HttpStatus,
    reason: Error,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An HTTP error occurred: {}", self.reason)
    }
}

impl From<Error> for HttpError {
    fn from(error: Error) -> HttpError {
        HttpError {
            status: match &error {
                Error::HeaderNotFound(_) | Error::JsonFailed(_) | Error::InvalidPayload(_) => {
                    HttpStatus::BadRequest
                }
                Error::VerificationFailed(_) => HttpStatus::Unauthorized,
                _ => HttpStatus::InternalServerError,
            },
            reason: error,
        }
    }
}
