use crate::model::ApprovalStatus;
use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};
use std::{convert::TryFrom, fmt};

struct ApprovalStatusVisitor;

impl<'de> Visitor<'de> for ApprovalStatusVisitor {
    type Value = ApprovalStatus;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i8 or a string")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "4" | "loved" => Ok(ApprovalStatus::Loved),
            "3" | "qualified" => Ok(ApprovalStatus::Qualified),
            "2" | "approved" => Ok(ApprovalStatus::Approved),
            "1" | "ranked" => Ok(ApprovalStatus::Ranked),
            "0" | "pending" => Ok(ApprovalStatus::Pending),
            "-1" | "wip" => Ok(ApprovalStatus::WIP),
            "-2" | "graveyard" => Ok(ApprovalStatus::Graveyard),
            _ => Err(Error::invalid_value(
                Unexpected::Str(v),
                &r#"
            "4", "loved",
            "3", qualified",
            "2", "approved",
            "1", "ranked",
            "0", "pending",
            "-1", "wip",
            "-2", or "graveyard"
            "#,
            )),
        }
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        ApprovalStatus::try_from(v as i8)
            .map_err(|_| Error::invalid_value(Unexpected::Signed(v), &"value between -2 and 4"))
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        ApprovalStatus::try_from(v as i8)
            .map_err(|_| Error::invalid_value(Unexpected::Unsigned(v), &"value between -2 and 4"))
    }
}

impl<'de> Deserialize<'de> for ApprovalStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ApprovalStatusVisitor)
    }
}
