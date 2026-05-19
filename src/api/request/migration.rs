//! Request types and builders for Migrations in Scryfall API
//!
//! More information can be found here: <https://scryfall.com/docs/api/migrations>
use std::str::FromStr;

use super::{BASE_URL, ScryfallApiError, ScryfallRequest};
use crate::objects::{CardMigration, List};

use url::Url;
use uuid::Uuid;

/// Request type for calling the `/migrations` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct MigrationRequest {
    page: u32,
}

impl MigrationRequest {
    /// Constructs a builder for a `MigrationRequest`
    pub fn builder() -> MigrationRequestBuilder {
        MigrationRequestBuilder::default()
    }
}

impl ScryfallRequest for MigrationRequest {
    type Response = List<CardMigration>;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url = url.join("/migrations")?;

        url.query_pairs_mut()
            .append_pair("page", &self.page.to_string());

        Ok(url)
    }
}

/// Builder for constructing a [`MigrationRequest`]
#[derive(Default)]
pub struct MigrationRequestBuilder {
    page: u32,
}

impl MigrationRequestBuilder {
    /// Constructs a new `MigrationRequestBuilder`
    pub fn new(page: u32) -> Self {
        Self { page }
    }

    /// Sets the `page` number to request for the Migration
    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Builds the [`MigrationRequest`]
    pub fn build(self) -> Result<MigrationRequest, ScryfallApiError> {
        Ok(MigrationRequest { page: self.page })
    }
}

/// Request type for calling the `/migrations/:id` Scryfall endpoint
#[derive(Debug, Clone)]
pub struct MigrationByIdRequest {
    id: Uuid,
}

impl MigrationByIdRequest {
    /// Constructs a builder for a `MigrationByIdRequest`
    pub fn builder() -> MigrationByIdRequestBuilder {
        MigrationByIdRequestBuilder::default()
    }
}

impl ScryfallRequest for MigrationByIdRequest {
    type Response = CardMigration;

    fn to_url(&self) -> Result<Url, ScryfallApiError> {
        let mut url = Url::parse(BASE_URL)?;
        let path = format!("/migrations/{}", self.id);
        url = url.join(&path)?;

        Ok(url)
    }
}

/// Builder for constructing a [`MigrationByIdRequest`]
pub struct MigrationByIdRequestBuilder {
    id: Uuid,
}

impl MigrationByIdRequestBuilder {
    /// Constructs a new `MigrationByIdRequestBuilder`
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    /// Sets the ID for the Migration to request
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    /// Builds the [`MigrationByIdRequest`]
    pub fn build(self) -> Result<MigrationByIdRequest, ScryfallApiError> {
        Ok(MigrationByIdRequest { id: self.id })
    }
}

impl Default for MigrationByIdRequestBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::from_str("6697b38a-ee19-455c-b24b-d0a659782d8b")
                .expect("valid migration id from Scryfall"),
        }
    }
}
