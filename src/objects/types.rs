use serde::{Deserialize, Serialize};

use crate::objects::{list::List, set::Set};

/// Exhaustive representation of a Mana Color in Magic: the Gathering
///
/// The main colors are `WUBRGC` with `T` being present on a single card but is still represented
/// here
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    /// White Mana
    #[serde(rename = "W")]
    White,

    /// Blue Mana
    #[serde(rename = "U")]
    Blue,

    /// Black Mana
    #[serde(rename = "B")]
    Black,

    /// Red Mana
    #[serde(rename = "R")]
    Red,

    /// Green Mana
    #[serde(rename = "G")]
    Green,

    /// Colorless Mana
    #[serde(rename = "C")]
    Colorless,

    /// Tap Mana (part of an un-set)
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

/// Exhaustive representation of a Mana Cost in Magic: the Gathering
///
/// All costs that are present in the game are represented by a combination of one or more of these
/// symbols
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostSymbol {
    /// Tap this Permanent
    #[serde(rename = "{T}")]
    Tap,

    /// Untap this Permanent
    #[serde(rename = "{Q}")]
    Untap,

    /// An Energy counter
    #[serde(rename = "{E}")]
    Energy,

    /// Modal budget pawprint
    #[serde(rename = "{P}")]
    Pawprint,

    /// Plansewalker
    #[serde(rename = "{PW}")]
    Planeswalker,

    /// Chaos cost
    #[serde(rename = "{CHAOS}")]
    Chaos,

    /// An Acorn counter
    #[serde(rename = "{A}")]
    Acorn,

    /// A Ticket counter (part of Unfinity)
    #[serde(rename = "{TK}")]
    Ticket,

    /// X Generic Mana cost
    #[serde(rename = "{X}")]
    XGeneric,

    /// Y Generic Mana cost
    #[serde(rename = "{Y}")]
    YGeneric,

    /// Z Generic Mana cost
    #[serde(rename = "{Z}")]
    ZGeneric,

    /// 0 Generic Mana cost
    #[serde(rename = "{0}")]
    Zero,

    /// ½ Generic Mana cost
    #[serde(rename = "{½}")]
    HalfGeneric,

    /// 1 Generic Mana cost
    #[serde(rename = "{1}")]
    Generic,
    /// 2 Generic Mana cost
    #[serde(rename = "{2}")]
    TwoGeneric,

    /// 3 Generic Mana cost
    #[serde(rename = "{3}")]
    ThreeGeneric,

    /// 4 Generic Mana cost
    #[serde(rename = "{4}")]
    FourGeneric,

    /// 5 Generic Mana cost
    #[serde(rename = "{5}")]
    FiveGeneric,

    /// 6 Generic Mana cost
    #[serde(rename = "{6}")]
    SixGeneric,

    /// 7 Generic Mana cost
    #[serde(rename = "{7}")]
    SevenGeneric,

    /// 8 Generic Mana cost
    #[serde(rename = "{8}")]
    EightGeneric,

    /// 9 Generic Mana cost
    #[serde(rename = "{9}")]
    NineGeneric,

    /// 10 Generic Mana cost
    #[serde(rename = "{10}")]
    TenGeneric,

    /// 11 Generic Mana cost
    #[serde(rename = "{11}")]
    ElevenGeneric,

    /// 12 Generic Mana cost
    #[serde(rename = "{12}")]
    TwelveGeneric,

    /// 13 Generic Mana cost
    #[serde(rename = "{13}")]
    ThirteenGeneric,

    /// 14 Generic Mana cost
    #[serde(rename = "{14}")]
    FourteenGeneric,

    /// 15 Generic Mana cost
    #[serde(rename = "{15}")]
    FifteenGeneric,

    /// 16 Generic Mana cost
    #[serde(rename = "{16}")]
    SixteenGeneric,

    /// 17 Generic Mana cost
    #[serde(rename = "{17}")]
    SeventeenGeneric,

    /// 18 Generic Mana cost
    #[serde(rename = "{18}")]
    EighteenGeneric,

    /// 19 Generic Mana cost
    #[serde(rename = "{19}")]
    NineteenGeneric,

    /// 20 Generic Mana cost
    #[serde(rename = "{20}")]
    TwentyGeneric,

    /// 100 Generic Mana cost
    #[serde(rename = "{100}")]
    HundredGeneric,

    /// 1000000 Generic Mana cost
    #[serde(rename = "{1000000}")]
    MillionGeneric,

    /// ∞ Generic Mana cost
    #[serde(rename = "{∞}")]
    InfinityGeneric,

    /// White or Blue Mana cost
    #[serde(rename = "{W/U}")]
    WhiteOrBlue,

    /// White or Black Mana cost
    #[serde(rename = "{W/B}")]
    WhiteOrBlack,

    /// Black or Red Mana cost
    #[serde(rename = "{B/R}")]
    BlackOrRed,

    /// Black or Green Mana cost
    #[serde(rename = "{B/G}")]
    BlackOrGreen,

    /// Blue or Black Mana cost
    #[serde(rename = "{U/B}")]
    BlueOrBlack,

    /// Blue or Red Mana cost
    #[serde(rename = "{U/R}")]
    BlueOrRed,

    /// Red or Green Mana cost
    #[serde(rename = "{R/G}")]
    RedOrGreen,

    /// Red or White mana cost
    #[serde(rename = "{R/W}")]
    RedOrWhite,

    /// Green or White Mana cost
    #[serde(rename = "{G/W}")]
    GreenOrWhite,

    /// Green or Blue Mana cost
    #[serde(rename = "{G/U}")]
    GreenOrBlue,

    /// Black or Green Mana cost or 2 Life
    #[serde(rename = "{B/G/P}")]
    BlackOrGreenPhyrexian,

    /// Black or Red Mana cost or 2 Life
    #[serde(rename = "{B/R/P}")]
    BlackOrRedPhyrexian,

    /// Green or Blue Mana cost or 2 Life
    #[serde(rename = "{G/U/P}")]
    GreenOrBluePhyrexian,

    /// Green or White Mana cost or 2 Life
    #[serde(rename = "{G/W/P}")]
    GreenOrWhitePhyrexian,

    /// Red or Green Mana cost or 2 Life
    #[serde(rename = "{R/G/P}")]
    RedOrGreenPhyrexian,

    /// Red or White Mana cost or 2 Life
    #[serde(rename = "{R/W/P}")]
    RedOrWhitePhyrexian,

    /// Blue or Black Mana cost or 2 Life
    #[serde(rename = "{U/B/P}")]
    BlueOrBlackPhyrexian,

    /// Blue or Red Mana cost or 2 Life
    #[serde(rename = "{U/R/P}")]
    BlueOrRedPhyrexian,

    /// White or Black Mana cost or 2 Life
    #[serde(rename = "{W/B/P}")]
    WhiteOrBlackPhyrexian,

    /// White or Blue Mana cost or 2 Life
    #[serde(rename = "{W/U/P}")]
    WhiteOrBluePhyrexian,

    /// Colorless or White Mana cost
    #[serde(rename = "{C/W}")]
    ColorlessOrWhite,

    /// Colorless or Blue Mana cost
    #[serde(rename = "{C/U}")]
    ColorlessOrBlue,

    /// Colorless or Black Mana cost
    #[serde(rename = "{C/B}")]
    ColorlessOrBlack,

    /// Colorless or Red Mana cost
    #[serde(rename = "{C/R}")]
    ColorlessOrRed,

    /// Colorless or Green Mana cost
    #[serde(rename = "{C/G}")]
    ColorlessOrGreen,

    /// 2 Generic Mana or White Mana cost
    #[serde(rename = "{2/W}")]
    TwoGenericOrWhite,

    /// 2 Generic Mana or Blue Mana cost
    #[serde(rename = "{2/U}")]
    TwoGenericOrBlue,

    /// 2 Generic Mana or Black Mana cost
    #[serde(rename = "{2/B}")]
    TwoGenericOrBlack,

    /// 2 Generic Mana or Red Mana cost
    #[serde(rename = "{2/R}")]
    TwoGenericOrRed,

    /// 2 Generic Mana or Green Mana cost
    #[serde(rename = "{2/G}")]
    TwoGenericOrGreen,

    /// 1 Colored Mana or 2 Life
    #[serde(rename = "{H}")]
    ColorOrPhyrexian,

    /// White Mana cost or 2 Life
    #[serde(rename = "{W/P}")]
    WhiteOrPhyrexian,

    /// Blue Mana cost or 2 Life
    #[serde(rename = "{U/P}")]
    BlueOrPhyrexian,

    /// Black Mana cost or 2 Life
    #[serde(rename = "{B/P}")]
    BlackOrPhyrexian,

    /// Red Mana cost or 2 Life
    #[serde(rename = "{R/P}")]
    RedOrPhyrexian,

    /// Green Mana cost or 2 Life
    #[serde(rename = "{G/P}")]
    GreenOrPhyrexian,

    /// Colorless Mana cost or 2 life
    #[serde(rename = "{C/P}")]
    ColorlessOrPhyrexian,

    /// Half White Mana cost (unset)
    #[serde(rename = "{HW}")]
    HalfWhite,

    /// Half Red Mana Cost (unset)
    #[serde(rename = "{HR}")]
    HalfRed,

    /// White Mana cost
    #[serde(rename = "{W}")]
    White,

    /// Blue Mana cost
    #[serde(rename = "{U}")]
    Blue,

    /// Black Mana cost
    #[serde(rename = "{B}")]
    Black,

    /// Red Mana cost
    #[serde(rename = "{R}")]
    Red,

    /// Green Mana cost
    #[serde(rename = "{G}")]
    Green,

    /// Colorless Mana cost
    #[serde(rename = "{C}")]
    Colorless,

    /// Snow Mana cost
    #[serde(rename = "{S}")]
    Snow,

    /// One Mana from a Legendary source
    #[serde(rename = "{L}")]
    Legendary,

    /// One potential Land Drop
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
            Self::SixteenGeneric => write!(f, "{{16}}"),
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

/// The Legal status of a Card in relation to a Game Format
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegalStatus {
    /// Legal in this format
    Legal,

    /// Illegal in this format
    NotLegal,

    /// Restricted to a single copy in this format
    Restricted,

    /// Banned in this format
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

/// The Border color of a Card
///
/// Some border formats are illegal in certain formats
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderColor {
    /// Black border
    Black,

    /// White border
    White,

    /// No border
    Borderless,

    /// Yellow border
    Yellow,

    /// Silver border
    Silver,

    /// Gold border
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

/// The "finish" of a Card
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardFinish {
    /// The Foil print (shiney)
    Foil,

    /// The Standard print
    NonFoil,

    /// The etched print
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

/// The card frame that can be present on a card
///
/// Several cards have older / modern card frames and some have prints in several frames
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardFrame {
    /// The initial card frame released in 1993
    #[serde(rename = "1993")]
    Frame1993,

    /// Updated card frame from 1997
    #[serde(rename = "1997")]
    Frame1997,

    /// Updated card frame from 2003
    #[serde(rename = "2003")]
    Frame2003,

    /// Updated card frame from 2015
    #[serde(rename = "2015")]
    Frame2015,

    /// Futuresight card frame from the `Futuresight` set
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
