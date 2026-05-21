#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

//! # Scryone
//!
//! Crate for providing an API Client, Types, and Request objects for working with the [Scryfall
//! API](https://scryfall.com/docs/api)
//!
//! It offers the following:
//!
//! * Types to represent Scryfall Objects (e.g. Cards, Sets, Rulings etc).
//! * Asynchronous and blocking API Clients to communicate with Scryfall
//! * Request types and builders for building requests to each of Scryfall API's endpoints
//!
//! ## Requests
//!
//! Scryfall has many endpoints for different aspects of their API.
//! To help you execute queries against certain endpoints, `scryone` provides Request types and
//! builders for them which allow you to set all parameters that Scryfall allows.
//!
//! Each Request type implements a trait called [`ScryfallRequest`][`api::ScryfallRequest`] which automatically builds the appropriate URL
//! for the API based on the parameters that you have set.
//!
//! For example, creating a request to search for a specific card (`/cards/search` endpoint in
//! Scryfall):
//!
//! ```rust
//! # use scryone::api::request::{CardSearchRequest, UniqueMode, SortOrder, SortDirection,
//! # DataFormat};
//! # use scryone::api::ScryfallApiError;
//! # fn main() -> Result<(), ScryfallApiError> {
//! let request = CardSearchRequest::builder()
//!     .query("Winter Orb")
//!     .unique_mode(UniqueMode::Cards)
//!     .sort_order(SortOrder::Set)
//!     .sort_direction(SortDirection::Desc)
//!     .data_format(DataFormat::Json)
//!     .build()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## API Clients
//!
//! An asynchronous and blocking client are supplied to enable you to contact Scryfall's API.
//!
//! The clients offer the following:
//!
//! * GET requests to the endpoint that accepts a [`ScryfallRequest`][`api::ScryfallRequest`] that
//! automatically constructs the URL (with parameters) for you.
//!
//! * POST requests to endpoints that allow it (only one single API endpoints supports this as of
//! now).
//!
//! * Generic GET requests that allow you to define the type you wish to deserialize into (you must
//! provide the URL)
//!
//! * Paginated requests that automatically (and respectfully) retrieve all data from paginated
//! responses from Scryfall.
//!
//! Example:
//!
//! ```rust,no_run
//! # use scryone::api::{ScryfallClient, ScryfallApiError, request::{CardSearchRequest, UniqueMode, SortOrder, SortDirection,
//! # DataFormat};
//! # async fn main() -> Result<(), ScryfallApiError> {
//! let client = ScryfallClient::new();
//! let request = CardSearchRequest::builder()
//!     .query("Winter Orb")
//!     .unique_mode(UniqueMode::Cards)
//!     .sort_order(SortOrder::Set)
//!     .sort_direction(SortDirection::Desc)
//!     .data_format(DataFormat::Json)
//!     .build()?;
//!
//! // Automatically constructs the appropriate URL for the `/cards/search` endpoint
//! let response = client.get(request).await?;
//! println!("Total cards: {}", response.total_cards);
//! println!("Cards: {:?}", response.data);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod objects;

#[cfg(test)]
mod parse_test {
    use super::*;

    use api::{
        ScryfallApiError,
        blocking::ScryfallClient,
        request::{BulkDataFromIdRequest, BulkDataId},
    };
    use objects::{BulkDataType, Card};

    #[test]
    fn deserialises_all_cards_successfully() -> Result<(), ScryfallApiError> {
        let client = ScryfallClient::new();
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(BulkDataType::OracleCards))
            .build()?;

        let response = client.get(req)?;
        let cards: Result<Vec<Card>, ScryfallApiError> = client.call(response.download_uri);
        if !cards.is_ok() {
            eprintln!("{cards:?}");
        }
        assert!(cards.is_ok());
        Ok(())
    }
}
