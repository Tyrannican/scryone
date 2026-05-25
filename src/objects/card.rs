//! Documentation relating to [`Card`] objects in Scryfall
//!
//! More detailed information can be found here: <https://scryfall.com/docs/api/cards>
use jiff::civil::Date;
use serde::{
    Deserialize, Serialize,
    de::{Deserializer, IntoDeserializer},
};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

use super::types::*;

/// Maps a String to a URI
pub type UriMap = HashMap<String, Url>;

/// # Card Objects
///
/// Card objects represent individual Magic: The Gathering cards that players could obtain and add
/// to their collection (with a few minor exceptions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// A Unique ID for the card in Scryfall's database
    pub id: Uuid,

    /// Content type for the object (always `card` for [`Card`] objects)
    pub object: String,

    /// A [`Language`] code for this printing
    pub lang: Language,

    /// A code for this card's [`Layout`]
    pub layout: Layout,

    /// A link to where you can begin paginating all re/prints for this card on Scryfall's API
    pub prints_search_uri: Url,

    /// A link to this card's rulings list on Scryfall's API
    pub rulings_uri: Url,

    /// A link to this card's permapage on Scryfall's website
    pub scryfall_uri: Url,

    /// A link to this card object on Scryfall's API
    pub uri: Url,

    /// A unique ID for this card's oracle identity.
    /// This value is consistent across reprinted card editions and unique among different cards
    /// with the same name (tokens, Unstable variants etc).
    /// Always present except for the [`Layout::ReversibleCard`] type where it is absent.
    /// `oracle_id` will be found on each face instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,

    /// This card's Arena ID, if any. A large percentage of cards are not available on Arena and do
    /// not have this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena_id: Option<u32>,

    /// This card's Magic Online ID (also known as the Catalog ID), if any. A large percentage of
    /// cards are not available on Magic Online and do not have this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_id: Option<u32>,

    /// This card's foil Magic Online ID (also known as the Catalog ID), if any. A large percentage
    /// of cards are not available on Magic Online and do not have this ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_foil_id: Option<u32>,

    /// This card's multiverse IDs on Gatherer, if any. Scryfall includes many promo cards
    /// tokens, and other esoteric object that do not have these identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiverse_ids: Option<Vec<u32>>,

    /// This card's Resource ID on Gatherer, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// This card's ID on [TCGPlayer's API](https://docs.tcgplayer.com/docs), also known as the
    /// `productId`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_id: Option<u32>,

    /// This card's ID on [TCGPlayer's API](https://docs.tcgplayer.com/docs), for its etched
    /// version if that version is a separate product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcgplayer_etched_id: Option<u32>,

    /// This card's ID on Cardmarket's API, also known as the `idProduct`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardmarket_id: Option<u32>,

    // Gameplay fields
    /// This card's mana value (Some `funny` cards have fractional costs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmc: Option<f32>,

    /// The name of this card. If this card has multiple faces, this field will contain both names
    /// separated by `␣//␣`
    pub name: String,

    /// Keywords that this card uses such as `Flying` and `Defender`
    pub keywords: Vec<String>,

    /// True if this card is on the [Reserved
    /// List](https://magic.wizards.com/en/news/announcements/official-reprint-policy)
    pub reserved: bool,

    /// The type line of this card (e.g. Artifact Creature - Human Artificer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_line: Option<String>,

    /// Describes the legality of this card across play formats ([`LegalStatus`])
    pub legalities: Legality,

    /// If this card is closely related to other cards, this will be an array of [`RelatedCard`]
    /// objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_parts: Option<Vec<RelatedCard>>,

    /// Set if the card is Multi-faced, an array of [`CardFace`] objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_faces: Option<Vec<CardFace>>,

    /// This card's [`Color`] identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_identity: Option<Vec<Color>>,

    /// The [`Color`] in this card's color indicator, if any. A null value for this field indicates
    /// the card has none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_indicator: Option<Vec<Color>>,

    /// This card's [`Color`], if the overall card has colors defined by the rules. Otherwise, the
    /// colors will be on the [`CardFace`] objects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Color>>,

    /// This face's defence, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<String>,

    /// This card's overall rank/popularity on EDHREC. Not all cards are ranked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edhrec_rank: Option<u32>,

    /// True if this card is on the [Commander Game Changer
    /// list](https://mtg.wiki/page/Commander_(format)/Game_Changers)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_changer: Option<bool>,

    /// This card's hand modifier, if it is Vanguard card. This value will contain a delta such as
    /// `-1`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_modifier: Option<String>,

    /// This card's life modifier, if it is Vanguard card. This value will contain a delta, such as
    /// `+2`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_modifier: Option<String>,

    /// This cards loyalty, if any. Note that some cards have loyalties that are non-numeric, such
    /// as `X`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loyalty: Option<String>,

    /// The mana cost for this card. This value will be any empty string `""` if the cost is
    /// absent. Multiface cards report this value in the [`CardFace`] objects.
    #[serde(
        default,
        deserialize_with = "parse_mana_cost_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub mana_cost: Option<Vec<CostSymbol>>,

    /// The Oracle text for this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_text: Option<String>,

    /// This card's rank/popularity on Penny Dreadful. Not all cards are ranked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penny_rank: Option<u32>,

    /// This card's power. Some cards have non-numeric values such as `*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,

    /// [`Color`] of mana that this card would produce
    #[serde(skip_serializing_if = "Option::is_none")]
    pub produced_mana: Option<Vec<Color>>,

    /// This card's toughness, if any. Some cards have non-numeric values such as `*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<String>,

    // Print Fields
    /// Whether this card is found in boosters
    pub booster: bool,

    /// The card's [`BorderColor`]
    pub border_color: BorderColor,

    /// This card's collector number. Some cards have non-numeric values, such as `*`
    pub collector_number: String,

    /// True if this card was only released in a digital format
    pub digital: bool,

    /// Collection of [`CardFinish`] flags to show the finishes this card comes with
    pub finishes: Vec<CardFinish>,

    /// This card's [`CardFrame`] layout
    pub frame: CardFrame,

    /// True if this card's art is larger than normal
    pub full_art: bool,

    /// List of [`GameType`] formats this card is available in
    pub games: Vec<GameType>,

    /// True if this card's imagery is high-resolution
    pub highres_image: bool,

    /// Indicator of the state of this card's image (see [`ImageStatus`])
    pub image_status: ImageStatus,

    /// True if this card is oversized
    pub oversized: bool,

    /// Object containing the daily price information for this card
    pub prices: PriceData,

    /// True if this card is a promotional print
    pub promo: bool,

    /// This card's [`Rarity`]
    pub rarity: Rarity,

    /// Object proficing URIs to this card's listing on other Magic: the Gathering online resources
    pub related_uris: UriMap,

    /// The date this card was released
    pub released_at: Date,

    /// True if this card is a reprint
    pub reprint: bool,

    /// A link to this card's set on the Scryfall website
    pub scryfall_set_uri: Url,

    /// The card's full set name
    pub set_name: String,

    /// A link to where you can begin paginating this card's set on the Scryfall API
    pub set_search_uri: Url,

    /// The type of set this printing is in
    pub set_type: String,

    /// A link to this card's [`Set`][`super::Set`] object on Scryfall's API
    pub set_uri: Url,

    /// This card's set code
    pub set: String,

    /// This car's Set object UUID
    pub set_id: Uuid,

    /// True if this card is a Story Spotlight
    pub story_spotlight: bool,

    /// True if the card is printed without text
    pub textless: bool,

    /// Whether this card is a variation of another printing
    pub variation: bool,

    /// The Scryfall ID for the card back design present on this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_back_id: Option<Uuid>,

    /// The name of the illustrator of this card. Newly spoiled cards may not have this field yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// The IDs of the artists that illustrated this card. Newly spoiled cards may not have this
    /// field yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_ids: Option<Vec<String>>,

    /// The lit [Unfinity Attractions](https://scryfall.com/search?q=t%3Aattraction+unique%3Aprints) lights on this card, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attraction_lights: Option<Vec<u32>>,

    /// True if you consider [avoiding use of this print](https://scryfall.com/blog/220) downstream
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_warning: Option<bool>,

    /// The just-for-fun name printed on the card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_name: Option<String>,

    /// The flavor text, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_text: Option<String>,

    /// This card's [`FrameEffect`], if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_effects: Option<Vec<FrameEffect>>,

    /// A unique identifer for the card artwork that remains consistent across reprints. Newly
    /// spoiled cards may not have this field yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,

    /// An object lsiting the available imagery for this card (see [`Image`])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uris: Option<Image>,

    /// The localised name printed on this card, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_name: Option<String>,

    /// The localised text printed on this card, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_text: Option<String>,

    /// The localised type line printed on this card, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_type_line: Option<String>,

    /// An array of strings describing what categories of promo cards this card falls into
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_types: Option<Vec<String>>,

    /// An object prividing URIs to this card's listing on major marketplaces. Omitted if this card
    /// is unpurchasable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_uris: Option<UriMap>,

    /// The printing ID of the printing this card is a variation of
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variation_of: Option<Uuid>,

    /// The security stamp on this card, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_stamp: Option<SecurityStamp>,

    /// This card's watermark, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,

    /// Preview information related to this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<PreviewInformation>,
}

/// Information relating to when a card was previewed and who by
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewInformation {
    /// The date this card was previewed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previewed_at: Option<Date>,

    /// A link to the preview for this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,

    /// The name of the source that previewed this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Card Face object
///
/// Some cards have multiple faces so this relates to a single face of a card
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardFace {
    /// The mana cost for this face. This value will be any empty string `""` if the cost is
    /// absent.
    #[serde(deserialize_with = "parse_mana_cost")]
    pub mana_cost: Vec<CostSymbol>,

    /// The name of this particular face
    pub name: String,

    /// A content type for this object, always `card_face`
    pub object: String,

    /// The name of the illustrator of this card face. Newly spoiled cards may not have this field
    /// yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,

    /// The ID of the illustrator of this card face. Newly spoiled cards may not have this field
    /// yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<Uuid>,

    /// The mana value of this particular face, if the card is reversible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmc: Option<f32>,

    /// The [`Color`] in thhis face's color indicator, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_indicator: Option<Vec<Color>>,

    /// This face's [`Color`] if defined for the individual face of this card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<Color>>,

    /// This face's defence, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<String>,

    /// The flavor test printed on this face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flavor_text: Option<String>,

    /// A unique identifier for the card face artwork that remains consistent across reprints.
    /// Newly spoiled cards may not have this field yet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,

    /// Object providing URIs to imagery for this face, if this is a double-sided card. If not
    /// double-sided, then this property will be on the parent object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uris: Option<Image>,

    /// The [`Layout`] of this card face, if the card is reversible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<Layout>,

    /// The Oracle ID of this particular face, if the card is reversible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,

    /// The Oracle text for this face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_text: Option<String>,

    /// This face's power, if any. Some cards have powers that are non-numeric such as `*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<String>,

    /// The localised name printed on this face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_name: Option<String>,

    /// The localised text printed on this face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_text: Option<String>,

    /// The lcoalised type line printed on this face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printed_type_line: Option<String>,

    /// This face's toughness if any. Some cards have toughnesses that are non-numeric such as `*`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toughness: Option<String>,

    /// The type line of this particular face, if the card is reversible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_line: Option<String>,

    /// The watermark on this particular card face, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<String>,
}

/// Related Card objects
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelatedCard {
    /// A unique ID for this card in Scryfall's database
    pub id: Uuid,

    /// A content type for this object, always `related_card`
    pub object: String,

    /// Field explaining what role this card plays in this relationship (see [`RelatedCardRole`]
    pub component: RelatedCardRole,

    /// The name of this particular related card
    pub name: String,

    /// The type line of this card
    pub type_line: String,

    /// A URI where you can retrieve a full object describing this card on Scryfall's API
    pub uri: Url,
}

/// Legal status of a card for each game format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legality {
    /// Standard format
    pub standard: LegalStatus,

    /// Future format
    pub future: LegalStatus,

    /// Historic format (Arena)
    pub historic: LegalStatus,

    /// Timeless format (Arena)
    pub timeless: LegalStatus,

    /// Gladiator format
    pub gladiator: LegalStatus,

    /// Pioneer format
    pub pioneer: LegalStatus,

    /// Modern format
    pub modern: LegalStatus,

    /// Legacy format
    pub legacy: LegalStatus,

    /// Pauper format
    pub pauper: LegalStatus,

    /// Vintage format
    pub vintage: LegalStatus,

    /// Penny-Dreadful format (MTGO)
    pub penny: LegalStatus,

    /// Commander format
    pub commander: LegalStatus,

    /// Oathbreaker format
    pub oathbreaker: LegalStatus,

    /// Standard Brawl format (Arena)
    pub standardbrawl: LegalStatus,

    /// Brawl format (Arena)
    pub brawl: LegalStatus,

    /// Alchemy format (Arena)
    pub alchemy: LegalStatus,

    /// Pauper Commander format
    pub paupercommander: LegalStatus,

    /// Duel format
    pub duel: LegalStatus,

    /// Old School format
    pub oldschool: LegalStatus,

    /// Pre-modern format
    pub premodern: LegalStatus,

    /// Pre-Commander Format
    pub predh: LegalStatus,

    /// Tiny-Leaders format
    pub tlr: LegalStatus,
}

/// Collection of Image URIs for different Image sizes / formats
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    /// Card in PNG format
    pub png: Url,

    /// Card without the Border
    pub border_crop: Url,

    /// Card with just the Artwork
    pub art_crop: Url,

    /// Card in its large size
    pub large: Url,

    /// Card in its normal size
    pub normal: Url,

    /// Card in its small size
    pub small: Url,
}

/// Price information for a given Card
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PriceData {
    /// Price in USD
    pub usd: Option<String>,

    /// Price in USD for the Foil variant
    pub usd_foil: Option<String>,

    /// Price in USD for the Etched format
    pub usd_etched: Option<String>,

    /// Price in EUR
    pub eur: Option<String>,

    /// Price in EUR for the Foil variant
    pub eur_foil: Option<String>,

    /// Price in EUR for the Etched format
    pub eur_etched: Option<String>,

    /// Price in MTGO Tickets
    pub tix: Option<String>,
}

/// Card Identifiers for searching for Collections in the API
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardIdentifier {
    /// Unique ID for a card in Scryfall's database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,

    /// MTGO ID for a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtgo_id: Option<u32>,

    /// Multiverse ID for a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiverse_id: Option<u32>,

    /// Oracle ID for the card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_id: Option<Uuid>,

    /// Illustration ID for a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub illustration_id: Option<Uuid>,

    /// Name of a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The Set Code for a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<String>,

    /// Collector number for a card
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_number: Option<String>,
}

fn parse_mana_cost_opt<'de, D>(deserializer: D) -> Result<Option<Vec<CostSymbol>>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Option<String> = Option::deserialize(deserializer)?;
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(None);
    };

    if raw.contains("//") {
        return Ok(None);
    }

    raw.split_inclusive('}')
        .map(|c| CostSymbol::deserialize(c.into_deserializer()))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn parse_mana_cost<'de, D>(deserializer: D) -> Result<Vec<CostSymbol>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: Option<String> = Option::deserialize(deserializer)?;
    let Some(raw) = raw.filter(|s| !s.is_empty()) else {
        return Ok(Vec::new());
    };

    raw.split_inclusive('}')
        .map(|c| CostSymbol::deserialize(c.into_deserializer()))
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod card_object_tests {
    use crate::api::{
        ScryfallApiError,
        blocking::ScryfallClient,
        request::{BulkDataFromIdRequest, BulkDataId},
    };
    use crate::objects::{BulkDataType, Card};

    fn bulk_data_download(bdt: BulkDataType) -> Result<Vec<Card>, ScryfallApiError> {
        let client = ScryfallClient::new();
        let req = BulkDataFromIdRequest::builder()
            .data_type(BulkDataId::Type(bdt))
            .build()?;

        let response = client.get(req)?;
        client.call(response.download_uri)
    }

    #[test]
    #[ignore = "downloads oracle cards from Scryfall - significant download time"]
    fn oracle_cards() -> Result<(), ScryfallApiError> {
        let cards = bulk_data_download(BulkDataType::OracleCards);
        assert!(cards.is_ok());

        Ok(())
    }

    #[test]
    #[ignore = "downloads all cards from Scryfall - significant download time"]
    fn all_cards() -> Result<(), ScryfallApiError> {
        let cards = bulk_data_download(BulkDataType::AllCards);
        assert!(cards.is_ok());

        Ok(())
    }

    #[test]
    #[ignore = "downloads unique artwork cards from Scryfall - significant download time"]
    fn unique_artwork() -> Result<(), ScryfallApiError> {
        let cards = bulk_data_download(BulkDataType::UniqueArtwork);
        assert!(cards.is_ok());

        Ok(())
    }

    #[test]
    #[ignore = "downloads default cards from Scryfall - significant download time"]
    fn default_cards() -> Result<(), ScryfallApiError> {
        let cards = bulk_data_download(BulkDataType::DefaultCards);
        assert!(cards.is_ok());

        Ok(())
    }
}
