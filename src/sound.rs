use std::error::Error;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Sound {
    Schwa("ə"),
    AccentSchwa("ə"),
    SchwaR("ɚ"),
    AccentSchwaR("ɚ"),
    ShortA("æ"),
    LongA("eɪ"),
    UmlautA("ä"),
    DotA("ɑ"),
    Au("aʊ"),
    ShortE("ɛ"),
    LongE("i"),
    ShortI("ɪ"),
    LongI("aɪ"),
    LongO("oʊ"),
    DotO("ɔ"),
    Oi("ɔɪ"),
    UmlautU("ü"),
    DotU("ü"),
    B("b"),
    Ch("tʃ"),
    D("d"),
    F("f"),
    G("ɡ"),
    H("h"),
    Hw("ʍ"),
    J("dʒ"),
    K("k"),
    UnderlineK("k"),
    L("k"),
    SchwaL("əl"),
    M("m"),
    SchwaM("əm"),
    N("n"),
    SchwaN("ən"),
    Ng("ŋ"),
    P("p"),
    R("ɹ"),
    S("s"),
    Sh("ʃ"),
    T("t"),
    Th("θ"),
    UnderlineTh("θ"),
    V("v"),
    W("w"),
    Y("j"),
    Z("z"),
    Zh("ʒ"),
}

impl Sound {
    fn get_string(&self) -> &str {
        match self {
            Sound::Schwa(s) => s,
            Sound::AccentSchwa(s) => s,
        }
    }
}

impl std::str::FromStr for Sound {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Schwa" => Ok(Sound::Schwa),
            "AccentSchwa" => Ok(Sound::AccentSchwa),
            "SchwaR" => Ok(Sound::SchwaR),
            "AccentSchwaR" => Ok(Sound::AccentSchwaR),
            "ShortA" => Ok(Sound::ShortA),
            "LongA" => Ok(Sound::LongA),
            "UmlautA" => Ok(Sound::UmlautA),
            "DotA" => Ok(Sound::DotA),
            "Au" => Ok(Sound::Au),
            "ShortE" => Ok(Sound::ShortE),
            "LongE" => Ok(Sound::LongE),
            "ShortI" => Ok(Sound::ShortI),
            "LongI" => Ok(Sound::LongI),
            "LongO" => Ok(Sound::LongO),
            "DotO" => Ok(Sound::DotO),
            "Oi" => Ok(Sound::Oi),
            "UmlautU" => Ok(Sound::UmlautU),
            "DotU" => Ok(Sound::DotU),
            "B" => Ok(Sound::B),
            "Ch" => Ok(Sound::Ch),
            "D" => Ok(Sound::D),
            "F" => Ok(Sound::F),
            "G" => Ok(Sound::G),
            "H" => Ok(Sound::H),
            "Hw" => Ok(Sound::Hw),
            "J" => Ok(Sound::J),
            "K" => Ok(Sound::K),
            "UnderlineK" => Ok(Sound::UnderlineK),
            "L" => Ok(Sound::L),
            "SchwaL" => Ok(Sound::SchwaL),
            "M" => Ok(Sound::M),
            "SchwaM" => Ok(Sound::SchwaM),
            "N" => Ok(Sound::N),
            "SchwaN" => Ok(Sound::SchwaN),
            "Ng" => Ok(Sound::Ng),
            "P" => Ok(Sound::P),
            "R" => Ok(Sound::R),
            "S" => Ok(Sound::S),
            "Sh" => Ok(Sound::Sh),
            "T" => Ok(Sound::T),
            "Th" => Ok(Sound::Th),
            "UnderlineTh" => Ok(Sound::UnderlineTh),
            "V" => Ok(Sound::V),
            "W" => Ok(Sound::W),
            "Y" => Ok(Sound::Y),
            "Z" => Ok(Sound::Z),
            "Zh" => Ok(Sound::Zh),
            // Return an error if the string is not a valid variant
            _ => Err(format!("Invalid animal: {}", s).into()),
        }
    }
}