use serde::{Deserialize, Serialize};

use crate::objects::{list::List, set::Set};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "W")]
    White,
    #[serde(rename = "U")]
    Blue,
    #[serde(rename = "B")]
    Black,
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "G")]
    Green,
    #[serde(rename = "C")]
    Colorless,
    #[serde(rename = "T")]
    Tap,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::White => write!(f, "W"),
            Self::Blue => write!(f, "U"),
            Self::Black => write!(f, "B"),
            Self::Red => write!(f, "R"),
            Self::Green => write!(f, "G"),
            Self::Colorless => write!(f, "C"),
            Self::Tap => write!(f, "T"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostSymbol {
    #[serde(rename = "{T}")]
    Tap,
    #[serde(rename = "{Q}")]
    Untap,
    #[serde(rename = "{E}")]
    Energy,
    #[serde(rename = "{P}")]
    Pawprint,
    #[serde(rename = "{PW}")]
    Planeswalker,
    #[serde(rename = "{CHAOS}")]
    Chaos,
    #[serde(rename = "{A}")]
    Acorn,
    #[serde(rename = "{TK}")]
    Ticket,
    #[serde(rename = "{X}")]
    XGeneric,
    #[serde(rename = "{Y}")]
    YGeneric,
    #[serde(rename = "{Z}")]
    ZGeneric,
    #[serde(rename = "{0}")]
    Zero,
    #[serde(rename = "{½}")]
    HalfGeneric,
    #[serde(rename = "{1}")]
    Generic,
    #[serde(rename = "{2}")]
    TwoGeneric,
    #[serde(rename = "{3}")]
    ThreeGeneric,
    #[serde(rename = "{4}")]
    FourGeneric,
    #[serde(rename = "{5}")]
    FiveGeneric,
    #[serde(rename = "{6}")]
    SixGeneric,
    #[serde(rename = "{7}")]
    SevenGeneric,
    #[serde(rename = "{8}")]
    EightGeneric,
    #[serde(rename = "{9}")]
    NineGeneric,
    #[serde(rename = "{10}")]
    TenGeneric,
    #[serde(rename = "{11}")]
    ElevenGeneric,
    #[serde(rename = "{12}")]
    TwelveGeneric,
    #[serde(rename = "{13}")]
    ThirteenGeneric,
    #[serde(rename = "{14}")]
    FourteenGeneric,
    #[serde(rename = "{15}")]
    FifteenGeneric,
    #[serde(rename = "{16}")]
    SixteenGenric,
    #[serde(rename = "{17}")]
    SeventeenGeneric,
    #[serde(rename = "{18}")]
    EighteenGeneric,
    #[serde(rename = "{19}")]
    NineteenGeneric,
    #[serde(rename = "{20}")]
    TwentyGeneric,
    #[serde(rename = "{100}")]
    HundredGeneric,
    #[serde(rename = "{1000000}")]
    MillionGeneric,
    #[serde(rename = "{∞}")]
    InfinityGeneric,
    #[serde(rename = "{W/U}")]
    WhiteOrBlue,
    #[serde(rename = "{W/B}")]
    WhiteOrBlack,
    #[serde(rename = "{B/R}")]
    BlackOrRed,
    #[serde(rename = "{B/G}")]
    BlackOrGreen,
    #[serde(rename = "{U/B}")]
    BlueOrBlack,
    #[serde(rename = "{U/R}")]
    BlueOrRed,
    #[serde(rename = "{R/G}")]
    RedOrGreen,
    #[serde(rename = "{R/W}")]
    RedOrWhite,
    #[serde(rename = "{G/W}")]
    GreenOrWhite,
    #[serde(rename = "{G/U}")]
    GreenOrBlue,
    #[serde(rename = "{B/G/P}")]
    BlackOrGreenPhyrexian,
    #[serde(rename = "{B/R/P}")]
    BlackOrRedPhyrexian,
    #[serde(rename = "{G/U/P}")]
    GreenOrBluePhyrexian,
    #[serde(rename = "{G/W/P}")]
    GreenOrWhitePhyrexian,
    #[serde(rename = "{R/G/P}")]
    RedOrGreenPhyrexian,
    #[serde(rename = "{R/W/P}")]
    RedOrWhitePhyrexian,
    #[serde(rename = "{U/B/P}")]
    BlueOrBlackPhyrexian,
    #[serde(rename = "{U/R/P}")]
    BlueOrRedPhyrexian,
    #[serde(rename = "{W/B/P}")]
    WhiteOrBlackPhyrexian,
    #[serde(rename = "{W/U/P}")]
    WhiteOrBluePhyrexian,
    #[serde(rename = "{C/W}")]
    ColorlessOrWhite,
    #[serde(rename = "{C/U}")]
    ColorlessOrBlue,
    #[serde(rename = "{C/B}")]
    ColorlessOrBlack,
    #[serde(rename = "{C/R}")]
    ColorlessOrRed,
    #[serde(rename = "{C/G}")]
    ColorlessOrGreen,
    #[serde(rename = "{2/W}")]
    TwoGenericOrWhite,
    #[serde(rename = "{2/U}")]
    TwoGenericOrBlue,
    #[serde(rename = "{2/B}")]
    TwoGenericOrBlack,
    #[serde(rename = "{2/R}")]
    TwoGenericOrRed,
    #[serde(rename = "{2/G}")]
    TwoGenericOrGreen,
    #[serde(rename = "{H}")]
    ColorOrPhyrexian,
    #[serde(rename = "{W/P}")]
    WhiteOrPhyrexian,
    #[serde(rename = "{U/P}")]
    BlueOrPhyrexian,
    #[serde(rename = "{B/P}")]
    BlackOrPhyrexian,
    #[serde(rename = "{R/P}")]
    RedOrPhyrexian,
    #[serde(rename = "{G/P}")]
    GreenOrPhyrexian,
    #[serde(rename = "{C/P}")]
    ColorlessOrPhyrexian,
    #[serde(rename = "{HW}")]
    HalfWhite,
    #[serde(rename = "{HR}")]
    HalfRed,
    #[serde(rename = "{W}")]
    White,
    #[serde(rename = "{U}")]
    Blue,
    #[serde(rename = "{B}")]
    Black,
    #[serde(rename = "{R}")]
    Red,
    #[serde(rename = "{G}")]
    Green,
    #[serde(rename = "{C}")]
    Colorless,
    #[serde(rename = "{S}")]
    Snow,
    #[serde(rename = "{L}")]
    Legendary,
    #[serde(rename = "{D}")]
    LandDrop,
}

impl std::fmt::Display for CostSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tap => write!(f, "{{T}}"),
            Self::Untap => write!(f, "{{Q}}"),
            Self::Energy => write!(f, "{{E}}"),
            Self::Pawprint => write!(f, "{{P}}"),
            Self::Planeswalker => write!(f, "{{PW}}"),
            Self::Chaos => write!(f, "{{CHAOS}}"),
            Self::Acorn => write!(f, "{{A}}"),
            Self::Ticket => write!(f, "{{TK}}"),
            Self::XGeneric => write!(f, "{{X}}"),
            Self::YGeneric => write!(f, "{{Y}}"),
            Self::ZGeneric => write!(f, "{{Z}}"),
            Self::Zero => write!(f, "{{0}}"),
            Self::HalfGeneric => write!(f, "{{½}}"),
            Self::Generic => write!(f, "{{1}}"),
            Self::TwoGeneric => write!(f, "{{2}}"),
            Self::ThreeGeneric => write!(f, "{{3}}"),
            Self::FourGeneric => write!(f, "{{4}}"),
            Self::FiveGeneric => write!(f, "{{5}}"),
            Self::SixGeneric => write!(f, "{{6}}"),
            Self::SevenGeneric => write!(f, "{{7}}"),
            Self::EightGeneric => write!(f, "{{8}}"),
            Self::NineGeneric => write!(f, "{{9}}"),
            Self::TenGeneric => write!(f, "{{10}}"),
            Self::ElevenGeneric => write!(f, "{{11}}"),
            Self::TwelveGeneric => write!(f, "{{12}}"),
            Self::ThirteenGeneric => write!(f, "{{13}}"),
            Self::FourteenGeneric => write!(f, "{{14}}"),
            Self::FifteenGeneric => write!(f, "{{15}}"),
            Self::SixteenGenric => write!(f, "{{16}}"),
            Self::SeventeenGeneric => write!(f, "{{17}}"),
            Self::EighteenGeneric => write!(f, "{{18}}"),
            Self::NineteenGeneric => write!(f, "{{19}}"),
            Self::TwentyGeneric => write!(f, "{{20}}"),
            Self::HundredGeneric => write!(f, "{{100}}"),
            Self::MillionGeneric => write!(f, "{{1000000}}"),
            Self::InfinityGeneric => write!(f, "{{∞}}"),
            Self::WhiteOrBlue => write!(f, "{{W/U}}"),
            Self::WhiteOrBlack => write!(f, "{{W/B}}"),
            Self::BlackOrRed => write!(f, "{{B/R}}"),
            Self::BlackOrGreen => write!(f, "{{B/G}}"),
            Self::BlueOrBlack => write!(f, "{{U/B}}"),
            Self::BlueOrRed => write!(f, "{{U/R}}"),
            Self::RedOrGreen => write!(f, "{{R/G}}"),
            Self::RedOrWhite => write!(f, "{{R/W}}"),
            Self::GreenOrWhite => write!(f, "{{G/W}}"),
            Self::GreenOrBlue => write!(f, "{{G/U}}"),
            Self::BlackOrGreenPhyrexian => write!(f, "{{B/G/P}}"),
            Self::BlackOrRedPhyrexian => write!(f, "{{B/R/P}}"),
            Self::GreenOrBluePhyrexian => write!(f, "{{G/U/P}}"),
            Self::GreenOrWhitePhyrexian => write!(f, "{{G/W/P}}"),
            Self::RedOrGreenPhyrexian => write!(f, "{{R/G/P}}"),
            Self::RedOrWhitePhyrexian => write!(f, "{{R/W/P}}"),
            Self::BlueOrBlackPhyrexian => write!(f, "{{U/B/P}}"),
            Self::BlueOrRedPhyrexian => write!(f, "{{U/R/P}}"),
            Self::WhiteOrBlackPhyrexian => write!(f, "{{W/B/P}}"),
            Self::WhiteOrBluePhyrexian => write!(f, "{{W/U/P}}"),
            Self::ColorlessOrWhite => write!(f, "{{C/W}}"),
            Self::ColorlessOrBlue => write!(f, "{{C/U}}"),
            Self::ColorlessOrBlack => write!(f, "{{C/B}}"),
            Self::ColorlessOrRed => write!(f, "{{C/R}}"),
            Self::ColorlessOrGreen => write!(f, "{{C/G}}"),
            Self::TwoGenericOrWhite => write!(f, "{{2/W}}"),
            Self::TwoGenericOrBlue => write!(f, "{{2/U}}"),
            Self::TwoGenericOrBlack => write!(f, "{{2/B}}"),
            Self::TwoGenericOrRed => write!(f, "{{2/R}}"),
            Self::TwoGenericOrGreen => write!(f, "{{2/G}}"),
            Self::ColorOrPhyrexian => write!(f, "{{H}}"),
            Self::WhiteOrPhyrexian => write!(f, "{{W/P}}"),
            Self::BlueOrPhyrexian => write!(f, "{{U/P}}"),
            Self::BlackOrPhyrexian => write!(f, "{{B/P}}"),
            Self::RedOrPhyrexian => write!(f, "{{R/P}}"),
            Self::GreenOrPhyrexian => write!(f, "{{G/P}}"),
            Self::ColorlessOrPhyrexian => write!(f, "{{C/P}}"),
            Self::HalfWhite => write!(f, "{{HW}}"),
            Self::HalfRed => write!(f, "{{HR}}"),
            Self::White => write!(f, "{{W}}"),
            Self::Blue => write!(f, "{{U}}"),
            Self::Black => write!(f, "{{B}}"),
            Self::Red => write!(f, "{{R}}"),
            Self::Green => write!(f, "{{G}}"),
            Self::Colorless => write!(f, "{{C}}"),
            Self::Snow => write!(f, "{{S}}"),
            Self::Legendary => write!(f, "{{L}}"),
            Self::LandDrop => write!(f, "{{D}}"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalStatus {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

impl std::fmt::Display for LegalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legal => write!(f, "legal"),
            Self::NotLegal => write!(f, "not_legal"),
            Self::Restricted => write!(f, "restricted"),
            Self::Banned => write!(f, "banned"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderColor {
    Black,
    White,
    Borderless,
    Yellow,
    Silver,
    Gold,
}

impl std::fmt::Display for BorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "black"),
            Self::White => write!(f, "white"),
            Self::Borderless => write!(f, "borderless"),
            Self::Yellow => write!(f, "yellow"),
            Self::Silver => write!(f, "silver"),
            Self::Gold => write!(f, "gold"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardFinish {
    Foil,
    NonFoil,
    Etched,
}

impl std::fmt::Display for CardFinish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Foil => write!(f, "foil"),
            Self::NonFoil => write!(f, "nonfoil"),
            Self::Etched => write!(f, "etched"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardFrame {
    #[serde(rename = "1993")]
    Frame1993,
    #[serde(rename = "1997")]
    Frame1997,
    #[serde(rename = "2003")]
    Frame2003,
    #[serde(rename = "2015")]
    Frame2015,
    #[serde(rename = "future")]
    Future,
}

impl std::fmt::Display for CardFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Frame1993 => write!(f, "1993"),
            Self::Frame1997 => write!(f, "1997"),
            Self::Frame2003 => write!(f, "2003"),
            Self::Frame2015 => write!(f, "2015"),
            Self::Future => write!(f, "future"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Enchantment,
    Draft,
    Devoid,
    Tombstone,
    ColorShifted,
    Inverted,
    SunMoonDfc,
    CompassLandDfc,
    OriginPwDfc,
    MoonEldraziDfc,
    WaxingAndWaningMoonDfc,
    Showcase,
    ExtendedArt,
    Companion,
    Etched,
    Snow,
    Lesson,
    ShatteredGlass,
    ConvertDfc,
    FanDfc,
    UpsideDownDfc,
    Spree,
    FullArt,
}

impl std::fmt::Display for FrameEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legendary => write!(f, "legendary"),
            Self::Miracle => write!(f, "miracle"),
            Self::Enchantment => write!(f, "enchantment"),
            Self::Draft => write!(f, "draft"),
            Self::Devoid => write!(f, "devoid"),
            Self::Tombstone => write!(f, "tombstone"),
            Self::ColorShifted => write!(f, "colorshifted"),
            Self::Inverted => write!(f, "inverted"),
            Self::SunMoonDfc => write!(f, "sunmoondfc"),
            Self::CompassLandDfc => write!(f, "compasslanddfc"),
            Self::OriginPwDfc => write!(f, "originpwdfc"),
            Self::MoonEldraziDfc => write!(f, "mooneldrazidfc"),
            Self::WaxingAndWaningMoonDfc => write!(f, "waxingandwaningmoondfc"),
            Self::Showcase => write!(f, "showcase"),
            Self::ExtendedArt => write!(f, "extendedart"),
            Self::Companion => write!(f, "companion"),
            Self::Etched => write!(f, "etched"),
            Self::Snow => write!(f, "snow"),
            Self::Lesson => write!(f, "lesson"),
            Self::ShatteredGlass => write!(f, "shatteredglass"),
            Self::ConvertDfc => write!(f, "convertdfc"),
            Self::FanDfc => write!(f, "fandfc"),
            Self::UpsideDownDfc => write!(f, "upsidedowndfc"),
            Self::Spree => write!(f, "spree"),
            Self::FullArt => write!(f, "fullart"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    Paper,
    Arena,
    Mtgo,
    Astral,
    Sega,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paper => write!(f, "paper"),
            Self::Arena => write!(f, "arena"),
            Self::Mtgo => write!(f, "mtgo"),
            Self::Astral => write!(f, "astral"),
            Self::Sega => write!(f, "sega"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImageStatus {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "lowres")]
    LowRes,
    #[serde(rename = "highres_scan")]
    HighResScan,
}

impl std::fmt::Display for ImageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => write!(f, "missing"),
            Self::Placeholder => write!(f, "placeholder"),
            Self::LowRes => write!(f, "lowres"),
            Self::HighResScan => write!(f, "highres_scan"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common => write!(f, "common"),
            Self::Uncommon => write!(f, "uncommon"),
            Self::Rare => write!(f, "rare"),
            Self::Special => write!(f, "special"),
            Self::Mythic => write!(f, "mythic"),
            Self::Bonus => write!(f, "bonus"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "zhs")]
    SimplifiedChinese,
    #[serde(rename = "zht")]
    TraditionalChinese,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "grc")]
    AncientGreek,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "ph")]
    Phyrexian,
    #[serde(rename = "qya")]
    Quenya,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "en"),
            Self::Spanish => write!(f, "es"),
            Self::French => write!(f, "fr"),
            Self::German => write!(f, "de"),
            Self::Italian => write!(f, "it"),
            Self::Portuguese => write!(f, "pt"),
            Self::Japanese => write!(f, "ja"),
            Self::Korean => write!(f, "ko"),
            Self::Russian => write!(f, "ru"),
            Self::SimplifiedChinese => write!(f, "zhs"),
            Self::TraditionalChinese => write!(f, "zht"),
            Self::Hebrew => write!(f, "he"),
            Self::Latin => write!(f, "la"),
            Self::AncientGreek => write!(f, "grc"),
            Self::Arabic => write!(f, "ar"),
            Self::Sanskrit => write!(f, "sa"),
            Self::Phyrexian => write!(f, "ph"),
            Self::Quenya => write!(f, "qya"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    ModalDfc,
    Meld,
    Leveler,
    Class,
    Case,
    Saga,
    Adventure,
    Prepare,
    Mutate,
    Prototype,
    Battle,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
    ArtSeries,
    Reversible,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Split => write!(f, "split"),
            Self::Flip => write!(f, "flip"),
            Self::Transform => write!(f, "transform"),
            Self::ModalDfc => write!(f, "modal_dfc"),
            Self::Meld => write!(f, "meld"),
            Self::Leveler => write!(f, "leveler"),
            Self::Class => write!(f, "class"),
            Self::Case => write!(f, "case"),
            Self::Saga => write!(f, "saga"),
            Self::Adventure => write!(f, "adventure"),
            Self::Prepare => write!(f, "prepare"),
            Self::Mutate => write!(f, "mutate"),
            Self::Prototype => write!(f, "prototype"),
            Self::Battle => write!(f, "battle"),
            Self::Planar => write!(f, "planar"),
            Self::Scheme => write!(f, "scheme"),
            Self::Vanguard => write!(f, "vanguard"),
            Self::Token => write!(f, "token"),
            Self::DoubleFacedToken => write!(f, "double_faced_token"),
            Self::Emblem => write!(f, "emblem"),
            Self::Augment => write!(f, "augment"),
            Self::Host => write!(f, "host"),
            Self::ArtSeries => write!(f, "art_series"),
            Self::Reversible => write!(f, "reversible_card"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedCardRole {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

impl std::fmt::Display for RelatedCardRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Token => write!(f, "token"),
            Self::MeldPart => write!(f, "meld_part"),
            Self::MeldResult => write!(f, "meld_result"),
            Self::ComboPiece => write!(f, "combo_piece"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}

impl std::fmt::Display for SecurityStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Oval => write!(f, "oval"),
            Self::Triangle => write!(f, "triangle"),
            Self::Acorn => write!(f, "acorn"),
            Self::Circle => write!(f, "circle"),
            Self::Arena => write!(f, "arena"),
            Self::Heart => write!(f, "heart"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Core,
    Expansion,
    Masters,
    Eternal,
    Alchemy,
    Masterpiece,
    Arsenal,
    FromTheVault,
    Spellbook,
    PremiumDeck,
    DuelDeck,
    DraftInnovation,
    TreasureChest,
    Commander,
    Planechase,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    Box,
    Promo,
    Token,
    Memorabilia,
    Minigame,
}

impl std::fmt::Display for SetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Core => write!(f, "core"),
            Self::Expansion => write!(f, "expansion"),
            Self::Masters => write!(f, "masters"),
            Self::Eternal => write!(f, "eternal"),
            Self::Alchemy => write!(f, "alchemy"),
            Self::Masterpiece => write!(f, "masterpiece"),
            Self::Arsenal => write!(f, "arsenal"),
            Self::FromTheVault => write!(f, "from_the_vault"),
            Self::Spellbook => write!(f, "spellbook"),
            Self::PremiumDeck => write!(f, "premium_deck"),
            Self::DuelDeck => write!(f, "duel_deck"),
            Self::DraftInnovation => write!(f, "draft_innovation"),
            Self::TreasureChest => write!(f, "treasure_chest"),
            Self::Commander => write!(f, "commander"),
            Self::Planechase => write!(f, "planechase"),
            Self::Archenemy => write!(f, "archenemy"),
            Self::Vanguard => write!(f, "vanguard"),
            Self::Funny => write!(f, "funny"),
            Self::Starter => write!(f, "starter"),
            Self::Box => write!(f, "box"),
            Self::Promo => write!(f, "promo"),
            Self::Token => write!(f, "token"),
            Self::Memorabilia => write!(f, "memorabilia"),
            Self::Minigame => write!(f, "minigame"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MigrationPlan {
    Merge,
    Delete,
}

impl std::fmt::Display for MigrationPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Merge => write!(f, "merge"),
            Self::Delete => write!(f, "delete"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RulingSource {
    Wotc,
    Scryfall,
}

impl std::fmt::Display for RulingSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wotc => write!(f, "wotc"),
            Self::Scryfall => write!(f, "scryfall"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BulkDataType {
    #[default]
    OracleCards,
    UniqueArtwork,
    DefaultCards,
    AllCards,
    Rulings,
}

impl std::fmt::Display for BulkDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OracleCards => write!(f, "oracle_cards"),
            Self::UniqueArtwork => write!(f, "unique_artwork"),
            Self::DefaultCards => write!(f, "default_cards"),
            Self::AllCards => write!(f, "all_cards"),
            Self::Rulings => write!(f, "rulings"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum MultiVariant {
    Set(Set),
    List(List<Set>),
}
