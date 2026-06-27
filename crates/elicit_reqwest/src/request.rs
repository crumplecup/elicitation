//! Shadow for `reqwest::Request`.

use elicitation::Elicit;
use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Body, Error, HeaderMap, Method, Url, Version};

/// Owned `reqwest::Request` shadow.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP request snapshot:")]
#[to_code_literal(path = "::elicit_reqwest::Request::from_parts", tuple)]
pub struct Request {
    #[prompt("HTTP method:")]
    method: Method,
    #[prompt("Parsed request URL:")]
    url: Url,
    #[prompt("HTTP headers:")]
    headers: HeaderMap,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional request body:")]
    body: Option<Body>,
    #[prompt("HTTP protocol version:")]
    version: Version,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Per-request timeout, when configured:")]
    timeout: Option<std::time::Duration>,
}

impl Request {
    /// Construct a new request snapshot with default headers/version.
    pub fn new(method: Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: HeaderMap::from(http::HeaderMap::new()),
            body: None,
            version: Version::from(reqwest::Version::default()),
            timeout: None,
        }
    }

    /// Construct a request snapshot from explicit parts.
    pub fn from_parts(
        method: Method,
        url: Url,
        headers: HeaderMap,
        body: Option<Body>,
        version: Version,
        timeout: Option<std::time::Duration>,
    ) -> Self {
        Self {
            method,
            url,
            headers,
            body,
            version,
            timeout,
        }
    }

    /// Rebuild a raw `reqwest::Request`.
    pub fn build_raw(&self) -> reqwest::Request {
        let mut request = reqwest::Request::new(
            reqwest::Method::from(self.method.clone()),
            (*self.url).clone(),
        );
        *request.headers_mut() = http::HeaderMap::from(self.headers.clone());
        *request.version_mut() = reqwest::Version::from(self.version.clone());
        *request.timeout_mut() = self.timeout;
        *request.body_mut() = self.body.clone().map(|body| body.build_raw());
        request
    }
}

impl TryFrom<reqwest::Request> for Request {
    type Error = Error;

    fn try_from(mut value: reqwest::Request) -> Result<Self, Self::Error> {
        let body = value.body_mut().take().map(Body::try_from).transpose()?;

        Ok(Self::from_parts(
            Method::from(value.method().clone()),
            Url::from(value.url().clone()),
            HeaderMap::from(value.headers().clone()),
            body,
            Version::from(value.version()),
            value.timeout().copied(),
        ))
    }
}

#[reflect_methods]
impl Request {
    /// Borrow the request method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Borrow the request URL.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Borrow the request headers.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Borrow the optional request body.
    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    /// Borrow the HTTP protocol version.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Borrow the per-request timeout, if set.
    pub fn timeout(&self) -> Option<std::time::Duration> {
        self.timeout
    }
}
