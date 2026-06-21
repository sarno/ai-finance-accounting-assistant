//! DTOs (Data Transfer Objects) — request/response structs for the API layer.
//! These are serializable and validated at the API boundary.

pub mod auth;
pub mod journal;
pub mod invoice;
pub mod payment;
pub mod report;
pub mod approval;
pub mod master_data;

pub mod date_format {
    use serde::{Serializer, Deserializer, Deserialize};
    use time::Date;

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let format = time::format_description::parse("[year]-[month]-[day]").map_err(serde::de::Error::custom)?;
        Date::parse(&s, &format).map_err(serde::de::Error::custom)
    }
}

pub mod option_date_format {
    use serde::{Serializer, Deserializer, Deserialize};
    use time::Date;

    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_some(&format!("{}", d)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(val) => {
                let format = time::format_description::parse("[year]-[month]-[day]").map_err(serde::de::Error::custom)?;
                let date = Date::parse(&val, &format).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}


