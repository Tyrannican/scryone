use std::str::FromStr;

use super::{BASE_URL, ScryfallApiError, ScryfallRequest};
use crate::objects::{CardMigration, List};

use url::Url;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MigrationRequest {
    page: u32,
}

impl MigrationRequest {
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

#[derive(Default)]
pub struct MigrationRequestBuilder {
    page: u32,
}

impl MigrationRequestBuilder {
    pub fn new(page: u32) -> Self {
        Self { page }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn build(self) -> Result<MigrationRequest, ScryfallApiError> {
        Ok(MigrationRequest { page: self.page })
    }
}

#[derive(Debug, Clone)]
pub struct MigrationByIdRequest {
    id: Uuid,
}

impl MigrationByIdRequest {
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

pub struct MigrationByIdRequestBuilder {
    id: Uuid,
}

impl MigrationByIdRequestBuilder {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

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
