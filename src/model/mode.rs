use std::fmt;

/// Available game modes
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GameMode {
    STD = 0,
    TKO = 1,
    CTB = 2,
    MNA = 3,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::STD
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::STD => f.write_str("osu"),
            Self::TKO => f.write_str("taiko"),
            Self::CTB => f.write_str("fruits"),
            Self::MNA => f.write_str("mania"),
        }
    }
}
