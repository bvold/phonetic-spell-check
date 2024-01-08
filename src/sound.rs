use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString, EnumIter, EnumMessage};

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
#[derive(Debug, Display, EnumString, EnumIter, EnumMessage, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Sound {
    #[strum(message = "ə")]
    Schwa,
    #[strum(message = "ə")]
    AccentSchwa,
    #[strum(message = "ɝ")]
    SchwaR,
    #[strum(message = "ɝ")]
    AccentSchwaR,
    #[strum(message = "æ")]
    ShortA,
    #[strum(message = "eɪ")]
    LongA,
    #[strum(message = "ɑ")]
    UmlautA,
    #[strum(message = "ɑ")]
    DotA,
    #[strum(message = "aʊ")]
    Au,
    #[strum(message = "ɛ")]
    ShortE,
    #[strum(message = "i")]
    LongE,
    #[strum(message = "ɪ")]
    ShortI,
    #[strum(message = "aɪ")]
    LongI,
    #[strum(message = "oʊ")]
    LongO,
    #[strum(message = "ɔ")]
    DotO,
    #[strum(message = "ɔɪ")]
    Oi,
    #[strum(message = "u")]
    LongU,
    #[strum(message = "ju")]
    UmlautU,
    #[strum(message = "u")]
    DotU,
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
    #[strum(message = "h")]
    H,
    #[strum(message = "ʍ")]
    Hw,
    #[strum(message = "dʒ")]
    J,
    #[strum(message = "k")]
    K,
    #[strum(message = "k")]
    UnderlineK,
    #[strum(message = "ɫ")]
    L,
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
    #[strum(message = "p")]
    P,
    #[strum(message = "ɹ")]
    R,
    #[strum(message = "s")]
    S,
    #[strum(message = "ʃ")]
    Sh,
    #[strum(message = "t")]
    T,
    #[strum(message = "θ")]
    Th,
    #[strum(message = "ð")]
    UnderlineTh,
    #[strum(message = "v")]
    V,
    #[strum(message = "w")]
    W,
    #[strum(message = "j")]
    Y,
    #[strum(message = "z")]
    Z,
    #[strum(message = "ʒ")]
    Zh,
    #[strum(message = "")]
    Silent
}
