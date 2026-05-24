//! Objects and types for Scryfall Data
pub mod bulk;
pub mod card;
pub mod catalog;
pub mod cost;
pub mod error;
pub mod list;
pub mod migration;
pub mod ruling;
pub mod set;
pub mod symbol;
pub mod types;

pub use bulk::*;
pub use card::*;
pub use catalog::*;
pub use cost::*;
pub use error::*;
pub use list::*;
pub use migration::*;
pub use ruling::*;
pub use set::*;
pub use symbol::*;
pub use types::*;

#[cfg(test)]
mod object_deserialisation_tests {
    use super::*;
    use serde::de::DeserializeOwned;

    fn deserialise<T: DeserializeOwned>(input: &str) {
        let obj = serde_json::from_str::<T>(&input);
        assert!(obj.is_ok());
    }

    #[test]
    fn bulk_data() {
        let raw = r#"    {
      "object": "bulk_data",
      "id": "27bf3214-1271-490b-bdfe-c0be6c23d02e",
      "type": "oracle_cards",
      "updated_at": "2026-05-23T09:02:54.512+00:00",
      "uri": "https://api.scryfall.com/bulk-data/27bf3214-1271-490b-bdfe-c0be6c23d02e",
      "name": "Oracle Cards",
      "description": "A JSON file containing one Scryfall card object for each Oracle ID on Scryfall. The chosen sets for the cards are an attempt to return the most up-to-date recognizable version of the card.",
      "size": 173116018,
      "download_uri": "https://data.scryfall.io/oracle-cards/oracle-cards-20260523090254.json",
      "content_type": "application/json",
      "content_encoding": "gzip"
    }"#;

        let types = vec![
            "oracle_cards",
            "unique_artwork",
            "default_cards",
            "all_cards",
            "rulings",
        ];

        for t in types {
            let raw = raw.replace(
                "\"type\": \"oracle_cards\",",
                &format!("\"type\": \"{t}\","),
            );

            deserialise::<BulkData>(&raw);
        }
    }

    #[test]
    fn catalog() {
        let raw = r#"{
  "object": "catalog",
  "uri": "https://api.scryfall.com/catalog/land-types",
  "total_values": 18,
  "data": [
    "Cave",
    "Cloud",
    "Desert",
    "Forest",
    "Gate",
    "Island",
    "Lair",
    "Locus",
    "Mine",
    "Mountain",
    "Sphere",
    "Plains",
    "Planet",
    "Power-Plant",
    "Swamp",
    "Tower",
    "Town",
    "Urza's"
  ]
}"#;

        deserialise::<Catalog>(&raw);
    }

    #[test]
    fn mana_cost() {
        let raw = r#"{
  "object": "mana_cost",
  "cost": "{X}{U}{R}",
  "colors": [
    "U",
    "R"
  ],
  "cmc": 2,
  "colorless": false,
  "monocolored": false,
  "multicolored": true
}"#;

        deserialise::<ManaCost>(&raw);
    }

    #[test]
    fn error() {
        let raw = r#"{
  "object": "error",
  "code": "bad_request",
  "status": 400,
  "warnings": [
    "Invalid expression “is:slick” was ignored. Checking if cards are “slick” is not supported",
    "Invalid expression “cmc>cmc” was ignored. The sides of your comparison must be different."
  ],
  "details": "All of your terms were ignored."
}"#;

        deserialise::<ScryfallError>(&raw);
    }

    #[test]
    fn list() {
        let raw = r#"{
  "object": "list",
  "total_cards": 1,
  "has_more": true,
  "next_page": "https://api.scryfall.com/cards/search?format=json&include_extras=false&include_multilingual=false&include_variations=false&order=name&page=2&q=c%3Awhite+mv%3D1&unique=cards",
  "data": [
    {
      "object": "card",
      "id": "0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd",
      "oracle_id": "92b617f9-6e6b-42a0-9c45-6ff5f6193051",
      "multiverse_ids": [],
      "tcgplayer_id": 662289,
      "cardmarket_id": 844373,
      "name": "Aang's Defense",
      "lang": "en",
      "released_at": "2025-11-21",
      "uri": "https://api.scryfall.com/cards/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd",
      "scryfall_uri": "https://scryfall.com/card/tle/266/aangs-defense?utm_source=api",
      "layout": "normal",
      "highres_image": true,
      "image_status": "highres_scan",
      "image_uris": {
        "small": "https://cards.scryfall.io/small/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.jpg?1764119522",
        "normal": "https://cards.scryfall.io/normal/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.jpg?1764119522",
        "large": "https://cards.scryfall.io/large/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.jpg?1764119522",
        "png": "https://cards.scryfall.io/png/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.png?1764119522",
        "art_crop": "https://cards.scryfall.io/art_crop/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.jpg?1764119522",
        "border_crop": "https://cards.scryfall.io/border_crop/front/0/d/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd.jpg?1764119522"
      },
      "mana_cost": "{W}",
      "cmc": 1,
      "type_line": "Instant",
      "oracle_text": "Target blocking creature you control gets +2/+2 until end of turn.\nDraw a card.",
      "colors": [
        "W"
      ],
      "color_identity": [
        "W"
      ],
      "keywords": [],
      "legalities": {
        "standard": "not_legal",
        "future": "not_legal",
        "historic": "legal",
        "timeless": "legal",
        "gladiator": "legal",
        "pioneer": "not_legal",
        "modern": "not_legal",
        "legacy": "legal",
        "pauper": "legal",
        "vintage": "legal",
        "penny": "not_legal",
        "commander": "legal",
        "oathbreaker": "legal",
        "standardbrawl": "not_legal",
        "brawl": "legal",
        "alchemy": "not_legal",
        "paupercommander": "legal",
        "duel": "legal",
        "oldschool": "not_legal",
        "premodern": "not_legal",
        "predh": "not_legal",
        "tlr": "legal"
      },
      "games": [
        "paper",
        "arena",
        "mtgo"
      ],
      "reserved": false,
      "game_changer": false,
      "foil": false,
      "nonfoil": true,
      "finishes": [
        "nonfoil"
      ],
      "oversized": false,
      "promo": false,
      "reprint": false,
      "variation": false,
      "set_id": "a75e6ceb-d62c-4063-8e96-f269e3a0b025",
      "set": "tle",
      "set_name": "Avatar: The Last Airbender Eternal",
      "set_type": "eternal",
      "set_uri": "https://api.scryfall.com/sets/a75e6ceb-d62c-4063-8e96-f269e3a0b025",
      "set_search_uri": "https://api.scryfall.com/cards/search?order=set&q=e%3Atle&unique=prints",
      "scryfall_set_uri": "https://scryfall.com/sets/tle?utm_source=api",
      "rulings_uri": "https://api.scryfall.com/cards/0dbb9439-6a4c-482a-83e7-5d6aa80d0cbd/rulings",
      "prints_search_uri": "https://api.scryfall.com/cards/search?order=released&q=oracleid%3A92b617f9-6e6b-42a0-9c45-6ff5f6193051&unique=prints",
      "collector_number": "266",
      "digital": false,
      "rarity": "common",
      "watermark": "airnomads",
      "flavor_text": "\"I guess you've never fought an Airbender before.\"\n—Aang",
      "card_back_id": "0aeebaf5-8c7d-4636-9e82-8c27447861f7",
      "artist": "Jo Cordisco",
      "artist_ids": [
        "f9452ff6-516a-489e-9271-ce8ee876275e"
      ],
      "illustration_id": "0bc67678-c317-48ad-8e65-bc5629b2ad4c",
      "border_color": "black",
      "frame": "2015",
      "full_art": false,
      "textless": false,
      "booster": false,
      "story_spotlight": false,
      "promo_types": [
        "universesbeyond"
      ],
      "edhrec_rank": 16667,
      "prices": {
        "usd": "0.19",
        "usd_foil": null,
        "usd_etched": null,
        "eur": "0.24",
        "eur_foil": null,
        "tix": null
      },
      "related_uris": {
        "tcgplayer_infinite_articles": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Farticles&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Farticles%3FproductLineName%3Dmagic%26q%3DAang%2527s%2BDefense",
        "tcgplayer_infinite_decks": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&trafcat=tcgplayer.com%2Fsearch%2Fdecks&u=https%3A%2F%2Fwww.tcgplayer.com%2Fsearch%2Fdecks%3FproductLineName%3Dmagic%26q%3DAang%2527s%2BDefense",
        "edhrec": "https://edhrec.com/route/?cc=Aang%27s+Defense"
      },
      "purchase_uris": {
        "tcgplayer": "https://partner.tcgplayer.com/c/4931599/1830156/21018?subId1=api&u=https%3A%2F%2Fwww.tcgplayer.com%2Fproduct%2F662289%3Fpage%3D1",
        "cardmarket": "https://www.cardmarket.com/en/Magic/Products?idProduct=844373&referrer=scryfall&utm_campaign=card_prices&utm_medium=text&utm_source=scryfall",
        "cardhoarder": "https://www.cardhoarder.com/cards?affiliate_id=scryfall&data%5Bsearch%5D=Aang%27s+Defense&ref=card-profile&utm_campaign=affiliate&utm_medium=card&utm_source=scryfall"
      }
    }
    ]
    }"#;

        deserialise::<List<Card>>(&raw);
    }
}
