use crate::OsuError;

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{fmt, str::FromStr};

/// Enum for a [`Score`](crate::model::Score)'s grade (sometimes called rank)
#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, PartialOrd, Serialize)]
pub enum Grade {
    F,
    D,
    C,
    B,
    A,
    S,
    SH,
    X,
    XH,
}

impl Grade {
    /// Check two grades for equality, ignoring silver-/regular-S difference
    ///
    /// # Example
    /// ```
    /// use rosu_v2::model::Grade;
    ///
    /// assert!(Grade::S.eq_letter(Grade::SH));
    /// assert!(!Grade::X.eq_letter(Grade::SH));
    /// ```
    #[inline]
    pub fn eq_letter(self, other: Grade) -> bool {
        match self {
            Grade::XH | Grade::X => other == Grade::XH || other == Grade::X,
            Grade::SH | Grade::S => other == Grade::SH || other == Grade::S,
            _ => self == other,
        }
    }
}

impl FromStr for Grade {
    type Err = OsuError;

    fn from_str(grade: &str) -> Result<Self, Self::Err> {
        let grade = match grade.to_uppercase().as_str() {
            "XH" | "SSH" => Self::XH,
            "X" | "SS" => Self::X,
            "SH" => Self::SH,
            "S" => Self::S,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "F" => Self::F,
            _ => return Err(OsuError::ParsingValue { value: "Grade" }),
        };

        Ok(grade)
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct GradeVisitor;

impl<'de> Visitor<'de> for GradeVisitor {
    type Value = Grade;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Grade::from_str(v).map_err(|_| Error::invalid_value(Unexpected::Str(v), &"a grade string"))
    }
}

impl<'de> Deserialize<'de> for Grade {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(GradeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_eq() {
        assert!(Grade::SH.eq_letter(Grade::S));
    }

    #[test]
    fn grade_neq() {
        assert!(!Grade::S.eq_letter(Grade::A));
    }

    #[test]
    fn grade_ord() {
        assert!(Grade::S > Grade::A);
    }
}
