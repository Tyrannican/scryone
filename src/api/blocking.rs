//! Blocking Scryfall API Client
//!
//! The blocking `ScryfallClient` will block the current thread to execute instead of returning
//! futures that need to be executed on a runtime.
//!
//! This follows the same rules as [`reqwest::blocking::Client`] so the blocking client _must not_
//! be executed within an async runtime or it will panic when attempting to block.
//!
//! See [`reqwest::blocking`] documentation for more information

use crate::api::error::ScryfallApiError;

use super::ScryfallClient as AsyncScryfallClient;
use super::request::{PaginatedRequest, ScryfallPostRequest, ScryfallRequest};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use tokio::runtime::{Builder, Runtime};

/// Scryfall API Blocking Client
///
/// The Client can make requests to specific endpoints based on the [`ScryfallRequest`] that is
/// provided or can make generalised requests to a any endpoint.
///
/// The [`ScryfallRequest`] calls automatically construct the URL to that endpoint so you don't
/// have to and automatically deserialise the response to the expected format.
///
/// The generalised requests require a URL and for the response type to be given.
#[derive(Debug)]
pub struct ScryfallClient {
    inner: AsyncScryfallClient,
    rt: Runtime,
}

impl ScryfallClient {
    /// Construct a new `ScryfallClient`
    pub fn new() -> Self {
        let rt = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("a valid tokio runtime on the current thread");
        let client = AsyncScryfallClient::new();

        Self { inner: client, rt }
    }

    /// Call a generic endpoint and returns the raw bytes of the response
    pub fn call_raw(&self, url: impl IntoUrl) -> Result<Vec<u8>, ScryfallApiError> {
        self.rt.block_on(self.inner.call_raw(url))
    }

    /// Call a generic endpoint that has response type `T`
    ///
    /// This is useful for download endpoints to download bulk data
    pub fn call<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T, ScryfallApiError> {
        self.rt.block_on(self.inner.call(url))
    }

    /// Makes a paginaged GET request to a Scryfall endpoint depending on the `ScryfallRequest`
    /// provided
    ///
    /// This will work on requests that return a `List<T>` type and will follow the pages until
    /// there are no more and return the data in a single `Vec<T>`
    pub fn paginated<R>(
        &self,
        request: R,
    ) -> Result<Vec<<R::Response as PaginatedRequest>::Item>, ScryfallApiError>
    where
        R: ScryfallRequest,
        R::Response: PaginatedRequest,
    {
        self.rt.block_on(self.inner.paginated(request))
    }

    /// Makes a GET request to a Scryfall endpoint depending on the `ScryfallRequest` provided
    ///
    /// The request will automatically contruct the URL and query parameters based on the fields
    /// that are present when it was built
    pub fn get<R: ScryfallRequest>(&self, request: R) -> Result<R::Response, ScryfallApiError> {
        self.rt.block_on(self.inner.get(request))
    }

    /// Makes a POST request to a Scryfall endpoint
    ///
    /// The only endpoint that allows this currently is `/cards/collection`
    pub fn post<R: ScryfallRequest + ScryfallPostRequest>(
        &self,
        request: R,
    ) -> Result<R::Response, ScryfallApiError> {
        self.rt.block_on(self.inner.get(request))
    }
}

impl Default for ScryfallClient {
    fn default() -> Self {
        Self::new()
    }
}
