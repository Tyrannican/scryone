//! # Scryone

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
