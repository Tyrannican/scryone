//! Scryfall API Client
pub mod blocking;
pub mod error;
pub mod request;

pub use error::ScryfallApiError;
pub use request::{ScryfallPostRequest, ScryfallRequest};

use reqwest::{Client, IntoUrl, StatusCode};
use serde::de::DeserializeOwned;

use crate::objects::{List, error::ScryfallError};

const SCRYFALL_USER_AGENT: &str = "scryone-agent";
const ACCEPT_HEADER: &str = "application/json;q=0.9,*/*;q=0.8";

pub struct ScryfallClient {
    client: Client,
}

impl ScryfallClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

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

    pub async fn paginated_request<T: DeserializeOwned + Clone>(
        &self,
        url: impl IntoUrl,
    ) -> Result<Vec<T>, ScryfallApiError> {
        let mut results: Vec<T> = Vec::new();
        let mut url = url.into_url()?;
        loop {
            let list = self
                .client
                .get(url.clone())
                .header("User-Agent", SCRYFALL_USER_AGENT)
                .header("Accept", ACCEPT_HEADER)
                .send()
                .await?
                .json::<List<T>>()
                .await?;

            results.extend_from_slice(&list.data);

            if list.has_more {
                url = list
                    .next_page
                    .expect("has_more is set so url should also be present");
            } else {
                break;
            }
        }

        Ok(results)
    }

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
            return Ok(response.json::<R::Response>().await?);
        }

        let body: ScryfallError = response
            .json()
            .await
            .map_err(|_| ScryfallApiError::InvalidResponse)?;

        match status {
            StatusCode::NOT_FOUND => Err(ScryfallApiError::NotFound(body.details)),
            StatusCode::BAD_REQUEST => Err(ScryfallApiError::BadRequest(body.details)),
            _ => Err(ScryfallApiError::InvalidResponse),
        }
    }

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
            .map_err(|_| ScryfallApiError::InvalidResponse)?;

        match status {
            StatusCode::NOT_FOUND => Err(ScryfallApiError::NotFound(body.details)),
            StatusCode::BAD_REQUEST => Err(ScryfallApiError::BadRequest(body.details)),
            _ => Err(ScryfallApiError::InvalidResponse),
        }
    }
}
