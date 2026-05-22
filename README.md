# Scryone

A Rust API client and types library to interact with the [Scryfall](https://scryfall.com/) API.

Provides the following:

* Types to represent Scryfall Objects (e.g. Cards, Sets, Rulings etc).
* Asynchronous and blocking API Clients to communicate with Scryfall
* Request types and builders for building requests to each of Scryfall API's endpoints

## Example

If using the asynchronous clients, you'll need a runtime like `Tokio`

```toml
[dependencies]

scryone = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Requesting a random card:

```rust
use scryone::{
    objects::Card,
    api::{
        ScryfallClient, ScryfallApiError,
        request::{DataFormat, RandomCardRequest}
    }
};

#[tokio::main]
async fn main() -> Result<(), ScryfallApiError> {
    let client = ScryfallClient::new();
    let request = RandomCardRequest::builder()
        .data_format(DataFormat::Json)
        .pretty(true)
        .build()?;

    let result: Card = client.get(request).await?;
    println!("Card: {result:?}");
}
```

Searching for a card:

```rust
use scryone::{
    api::{ScryfallClient, ScryfallApiError, request::{CardSearchRequest, UniqueMode, SortOrder, SortDirection, DataFormat},
    objects::Card
};

#[tokio::main]
async fn main() -> Result<(), ScryfallApiError> {
    let client = ScryfallClient::new();
    let request = CardSearchRequest::builder()
        .query("Winter Orb")
        .unique_mode(UniqueMode::Cards)
        .sort_order(SortOrder::Set)
        .sort_direction(SortDirection::Desc)
        .data_format(DataFormat::Json)
        .build()?;

    let response: Vec<Card> = client.paginated(request).await?;
    println!("Total cards: {}", response.total_cards);
    println!("Cards: {:?}", response.data);

    Ok(())
}
```
