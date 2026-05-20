//! Scryfall API Clients and Request Types

pub mod blocking;
pub mod error;
pub mod request;

use std::{any::Any, time::Duration};

pub use error::ScryfallApiError;
pub use request::{ScryfallPostRequest, ScryfallRequest};

use reqwest::{Client, IntoUrl, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    api::request::PaginatedRequest,
    objects::{List, error::ScryfallError},
};

const SCRYFALL_USER_AGENT: &str = "scryone-agent";
const ACCEPT_HEADER: &str = "application/json;q=0.9,*/*;q=0.8";

/// Scryfall API Asynchronous Client
///
/// The Client can make requests to specific endpoints based on the [`ScryfallRequest`] that is
/// provided or can make generalised requests to a any endpoint.
///
/// The [`ScryfallRequest`] calls automatically construct the URL to that endpoint so you don't
/// have to and automatically deserialise the response to the expected format.
///
/// The generalised requests require a URL and for the response type to be given.
pub struct ScryfallClient {
    client: Client,
}

impl ScryfallClient {
    /// Construct a new `ScryfallClient`
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Call a generic endpoint that has response type `T`
    ///
    /// This is useful for download endpoints to download bulk data
    pub async fn call<T: DeserializeOwned>(
        &self,
        url: impl IntoUrl,
    ) -> Result<T, ScryfallApiError> {
        self.client
            .get(url)
            .header("User-Agent", SCRYFALL_USER_AGENT)
            .header("Accept", ACCEPT_HEADER)
            .send()
            .await?
            .json::<T>()
            .await
            .map_err(|e| ScryfallApiError::ApiCall(e))
    }

    /// Makes a paginaged GET request to a Scryfall endpoint depending on the `ScryfallRequest`
    /// provided
    ///
    /// This will work on requests that return a `List<T>` type and will follow the pages until
    /// there are no more and return the data in a single Vec<T>
    pub async fn paginated<R>(
        &self,
        request: R,
    ) -> Result<Vec<<R::Response as PaginatedRequest>::Item>, ScryfallApiError>
    where
        R: ScryfallRequest,
        R::Response: PaginatedRequest,
    {
        let mut data = Vec::new();
        let mut url = request.to_url()?;

        loop {
            let page: R::Response = self.call(url).await?;
            let (items, next_page) = page.parts();
            data.extend(items);
            match next_page {
                Some(u) => url = u,
                None => break,
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(data)
    }

    /// Makes a GET request to a Scryfall endpoint depending on the `ScryfallRequest` provided
    ///
    /// The request will automatically contruct the URL and query parameters based on the fields
    /// that are present when it was built
    pub async fn get<R: ScryfallRequest>(
        &self,
        request: R,
    ) -> Result<R::Response, ScryfallApiError> {
        let url = request.to_url()?;

        let response = self
            .client
            .get(url)
            .header("User-Agent", SCRYFALL_USER_AGENT)
            .header("Accept", ACCEPT_HEADER)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let t = response.json::<R::Response>().await?;
            todo!()
            // return Ok(response.json::<R::Response>().await?);
        }

        let body: ScryfallError = response
            .json()
            .await
            .map_err(|_| ScryfallApiError::InvalidResponse(status))?;

        match status {
            StatusCode::NOT_FOUND => Err(ScryfallApiError::NotFound(body.details)),
            StatusCode::BAD_REQUEST => Err(ScryfallApiError::BadRequest(body.details)),
            _ => Err(ScryfallApiError::InvalidResponse(status)),
        }
    }

    /// Makes a POST request to a Scryfall endpoint
    ///
    /// The only endpoint that allows this currently is `/cards/collection`
    pub async fn post<R: ScryfallRequest + ScryfallPostRequest>(
        &self,
        request: R,
    ) -> Result<R::Response, ScryfallApiError> {
        let url = request.to_url()?;
        let body = request.body();

        let response = self
            .client
            .post(url)
            .header("User-Agent", SCRYFALL_USER_AGENT)
            .header("Accept", ACCEPT_HEADER)
            .json(body)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            return Ok(response.json::<R::Response>().await?);
        }

        let body: ScryfallError = response
            .json()
            .await
            .map_err(|_| ScryfallApiError::InvalidResponse(status))?;

        match status {
            StatusCode::NOT_FOUND => Err(ScryfallApiError::NotFound(body.details)),
            StatusCode::BAD_REQUEST => Err(ScryfallApiError::BadRequest(body.details)),
            _ => Err(ScryfallApiError::InvalidResponse(status)),
        }
    }
}
