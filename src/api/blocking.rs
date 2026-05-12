use crate::api::error::ScryfallApiError;

use super::ScryfallClient as AsyncScryfallClient;
use super::request::{ScryfallPostRequest, ScryfallRequest};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use tokio::runtime::{Builder, Runtime};

pub struct ScryfallClient {
    inner: AsyncScryfallClient,
    rt: Runtime,
}

impl ScryfallClient {
    pub fn new() -> Self {
        let rt = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("a valid tokio runtime on the current thread");
        let client = AsyncScryfallClient::new();

        Self { inner: client, rt }
    }

    pub fn call<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T, ScryfallApiError> {
        self.rt.block_on(self.inner.call(url))
    }

    pub fn get<R: ScryfallRequest>(&self, request: R) -> Result<R::Response, ScryfallApiError> {
        self.rt.block_on(self.inner.get(request))
    }

    pub fn post<R: ScryfallRequest + ScryfallPostRequest>(
        &self,
        request: R,
    ) -> Result<R::Response, ScryfallApiError> {
        self.rt.block_on(self.inner.get(request))
    }
}
