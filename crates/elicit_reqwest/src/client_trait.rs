//! The [`HttpClient`] trait: a concrete, mockable surface for `elicit_reqwest` types.
//!
//! Using concrete [`Url`] and [`Method`] parameters (rather than generics) keeps
//! the trait object-safe and compatible with mockall's code generation.

use tracing::instrument;

use crate::{Method, RequestBuilder, Url};

/// Mockable HTTP client interface mirroring the `elicit_reqwest::Client` API.
///
/// All methods take a concrete [`Url`] rather than a generic `IntoUrl` so that
/// the trait is object-safe and can be mocked in tests without generic method
/// restrictions.
pub trait HttpClient {
    /// Start building a request with the given method and URL.
    fn request_url(&self, method: Method, url: Url) -> RequestBuilder;

    /// Start building a GET request.
    fn get_url(&self, url: Url) -> RequestBuilder;

    /// Start building a POST request.
    fn post_url(&self, url: Url) -> RequestBuilder;

    /// Start building a PUT request.
    fn put_url(&self, url: Url) -> RequestBuilder;

    /// Start building a DELETE request.
    fn delete_url(&self, url: Url) -> RequestBuilder;

    /// Start building a PATCH request.
    fn patch_url(&self, url: Url) -> RequestBuilder;

    /// Start building a HEAD request.
    fn head_url(&self, url: Url) -> RequestBuilder;
}

impl HttpClient for crate::Client {
    #[instrument(skip(self, url), level = "debug")]
    fn request_url(&self, method: Method, url: Url) -> RequestBuilder {
        self.request_url(method, url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn get_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::GET), url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn post_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::POST), url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn put_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::PUT), url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn delete_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::DELETE), url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn patch_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::PATCH), url)
    }

    #[instrument(skip(self, url), level = "debug")]
    fn head_url(&self, url: Url) -> RequestBuilder {
        self.request_url(Method::from(reqwest::Method::HEAD), url)
    }
}
