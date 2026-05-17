use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{
    Decode, Postgres,
    postgres::{PgTypeInfo, PgValueRef},
    types::Type,
};
use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

/// Formato para a exposição do [`OffsetDateTime`] através da API.
const FORMAT: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

/// Tipo genérico para representação e serding de `TIMESTAMPTZ`.
#[derive(Debug, Clone, Copy)]
pub struct GenericDateTime(OffsetDateTime);

impl Serialize for GenericDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let formatted = self.0.format(FORMAT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&formatted)
    }
}

impl<'de> Deserialize<'de> for GenericDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let dt = OffsetDateTime::parse(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(Self(dt))
    }
}

impl From<OffsetDateTime> for GenericDateTime {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl From<GenericDateTime> for OffsetDateTime {
    fn from(value: GenericDateTime) -> Self {
        value.0
    }
}

impl Type<Postgres> for GenericDateTime {
    fn type_info() -> PgTypeInfo {
        <OffsetDateTime as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <OffsetDateTime as Type<Postgres>>::compatible(ty)
    }
}

impl<'r> Decode<'r, Postgres> for GenericDateTime {
    fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let dt = OffsetDateTime::decode(value)?;
        Ok(Self(dt))
    }
}
