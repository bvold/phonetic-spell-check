use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumMessage, EnumString};

/// # Sound Enum
///
/// This module defines the `Sound` enum, representing various phonetic sounds.
///
/// ## Usage
///
// ```rust
// use Sound; // Assuming the enum is defined in a module named "sound"
//
// let sound = Sound::ShortA;
// println!("{}", sound); // Prints "ShortA"
//
//
// ## Features
//
// - Serialization and deserialization using Serde.
// - String representation, iteration, and custom error messages using strum_macros.
// - Debug formatting for debugging.
// - Equality comparison and hashing for efficient use in collections.
//
// ## Variants
//
// The enum includes variants for various vowel and consonant sounds, as well as a silent variant.
// See the variant definitions below for details.
#[derive(
    Debug,
    Display,
    EnumString,
    EnumIter,
    EnumMessage,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Sound {
    #[strum(message = "ə")]
    Schwa,
    #[strum(message = "əw")]
    SchwaAndW,
    #[strum(message = "ə")]
    AccentSchwa,
    #[strum(message = "ɝ")]
    SchwaR,
    #[strum(message = "ɝ")]
    AccentSchwaR,
    #[strum(message = "æ")]
    ShortA,
    #[strum(message = "æɫ")]
    ShortAandL,
    #[strum(message = "eɪ")]
    LongA,
    #[strum(message = "eɪɑ")]
    LongAandDotA,
    #[strum(message = "ɑ")]
    UmlautA,
    #[strum(message = "ɑh")]
    UmlautAandH,
    #[strum(message = "ɑ")]
    DotA,
    #[strum(message = "ɑɫ")]
    DotAandL,
    #[strum(message = "aʊ")]
    Au,
    #[strum(message = "eɪɫ")]
    LongAandL,
    #[strum(message = "ɛɹ")]
    LongAandR,
    #[strum(message = "ɑɹ")]
    ShortAandR,
    #[strum(message = "id")]
    LongEandD,
    #[strum(message = "iɹ")]
    LongEandR,
    #[strum(message = "aɪɫ")]
    LongIandL,
    #[strum(message = "aɪɝ")]
    LongIandR,
    #[strum(message = "ɪɹ")]
    ShortIandR,
    #[strum(message = "ɛ")]
    ShortE,
    #[strum(message = "ɛd")]
    ShortEandD,
    #[strum(message = "i")]
    LongE,
    #[strum(message = "ɪ")]
    ShortI,
    #[strum(message = "aɪ")]
    LongI,
    #[strum(message = "aɪoʊ")]
    LongIandLongO,
    #[strum(message = "oʊ")]
    LongO,
    #[strum(message = "ɔ")]
    DotO,
    #[strum(message = "ɔɹ")]
    DotOandR,
    #[strum(message = "ɔɪ")]
    Oi,
    #[strum(message = "u")]
    LongU,
    #[strum(message = "uj")]
    LongUandY,
    #[strum(message = "jʊɹ")]
    UmlautUandR,
    #[strum(message = "ju")]
    UmlautU,
    #[strum(message = "ʊ")]
    DotU,
    #[strum(message = "ʊɹ")]
    DotUandR,
    #[strum(message = "b")]
    B,
    #[strum(message = "tʃ")]
    Ch,
    #[strum(message = "d")]
    D,
    #[strum(message = "f")]
    F,
    #[strum(message = "ɡ")]
    G,
    #[strum(message = "ɡaɪ")]
    GandLongI,
    #[strum(message = "h")]
    H,
    #[strum(message = "ʍ")]
    Hw,
    #[strum(message = "dʒ")]
    J,
    #[strum(message = "dʒə")]
    JandSchwa,
    #[strum(message = "dʒɪ")]
    JandShortI,
    #[strum(message = "dʒu")]
    JandLongU,
    #[strum(message = "k")]
    K,
    #[strum(message = "kə")]
    KandSchwa,
    #[strum(message = "kt")]
    KandT,
    #[strum(message = "kʊ")]
    KandUmlautU,
    #[strum(message = "kw")]
    KandW,
    #[strum(message = "k")]
    UnderlineK,
    #[strum(message = "ɫ")]
    L,
    #[strum(message = "ɫf")]
    LandF,
    #[strum(message = "ɫd")]
    LandD,
    #[strum(message = "ɫi")]
    LandLongE,
    #[strum(message = "ɫə")]
    LandSchwa,
    #[strum(message = "ɫɛ")]
    LandShortE,
    #[strum(message = "əɫ")]
    SchwaL,
    #[strum(message = "m")]
    M,
    #[strum(message = "əm")]
    SchwaM,
    #[strum(message = "n")]
    N,
    #[strum(message = "ən")]
    SchwaN,
    #[strum(message = "ŋ")]
    Ng,
    #[strum(message = "ŋɡ")]
    NgandG,
    #[strum(message = "p")]
    P,
    #[strum(message = "pt")]
    PandT,
    #[strum(message = "ɹ")]
    R,
    #[strum(message = "s")]
    S,
    #[strum(message = "sk")]
    SandK,
    #[strum(message = "ʃ")]
    Sh,
    #[strum(message = "t")]
    T,
    #[strum(message = "tə")]
    TandSchwa,
    #[strum(message = "taɪ")]
    TandLongI,
    #[strum(message = "tɪ")]
    TandShortI,
    #[strum(message = "tz")]
    TandZ,
    #[strum(message = "θ")]
    Th,
    #[strum(message = "ð")]
    UnderlineTh,
    #[strum(message = "v")]
    V,
    #[strum(message = "w")]
    W,
    #[strum(message = "ks")]
    X,
    #[strum(message = "j")]
    Y,
    #[strum(message = "jʊ")]
    Yu,
    #[strum(message = "z")]
    Z,
    #[strum(message = "ʒ")]
    Zh,
    #[strum(message = "")]
    Silent,
}
