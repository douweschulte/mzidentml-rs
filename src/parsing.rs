/// Splits up a CV ID like <ONTHOLOGY>:<ID_NUM> into (String, usize).
/// If the number part is not parsable, ("<ONTHOLOGY>", 0) is returned
/// to not interrupt the parsing.
pub mod cv_id_parsing {

    use serde::{self, Deserialize, Deserializer, Serializer};

    use crate::{error::CvError, utils::split_accession};

    pub fn serialize<S>(accession: &(String, usize), serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(format!("{}:{}", accession.0, accession.1).as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<(String, usize), D::Error>
    where
        D: Deserializer<'de>,
    {
        let id: String = Deserialize::deserialize(deserializer)?;
        let (cv_name, id): (String, usize) = match split_accession(&id) {
            Ok((cv_name, id)) => (cv_name.into(), id),
            Err(err) => match err {
                CvError::InvalidSecondPart(cv, _) => (cv, 0),
                _ => {
                    return Err(serde::de::Error::custom(format!(
                        "Unable to parse CV accession {id}, {err}"
                    )));
                }
            },
        };

        Ok((cv_name, id))
    }
}

pub mod opt_date_time_parsing {
    use chrono::{DateTime, FixedOffset};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(
        activity_date: &Option<DateTime<FixedOffset>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match activity_date {
            Some(date) => serializer.serialize_some(&Some(date.to_rfc3339())),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str: Option<String> = Deserialize::deserialize(deserializer)?;

        if date_str.is_none() {
            return Ok(None);
        }

        let datetime = DateTime::parse_from_rfc3339(date_str.as_ref().unwrap()).map_err(|err| {
            serde::de::Error::custom(format!(
                "Failed to parse '{}' as RFC3339 date: {err:?}",
                date_str.unwrap()
            ))
        })?;

        Ok(Some(datetime))
    }
}

// TODO: Use serializer.serialize_seq() & deserializer.deserialize_seq()
pub mod space_separated_vec_parsing {
    use std::str::FromStr;

    use serde::{self, Deserialize, Deserializer, Serializer};

    pub static ITEM_SEPARATOR: &str = " ";

    pub fn serialize<S, T>(items: &[T], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        let items_str = items
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(ITEM_SEPARATOR);

        serializer.serialize_str(&items_str)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let items_str: String = Deserialize::deserialize(deserializer)?;

        items_str
            .split(ITEM_SEPARATOR)
            .map(|item| T::from_str(item))
            .collect::<Result<Vec<_>, T::Err>>()
            .map_err(|err| {
                serde::de::Error::custom(format!("Unable to parse space separated list {err:?}"))
            })
    }
}

/// Parses optional space separated vectors from and to string.
/// An empty string will be parsed to None.
// TODO: Possible to merge with space_separated_vec_parsing?
pub mod opt_space_separated_vec_parsing {
    use std::str::FromStr;

    use serde::{self, Deserialize, Deserializer, Serializer};

    pub static ITEM_SEPARATOR: &str = " ";

    pub fn serialize<S, T>(items: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        if items.is_none() {
            return serializer.serialize_none();
        }

        let items_str = items
            .as_ref()
            .unwrap()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(ITEM_SEPARATOR);

        serializer.serialize_some(&Some(items_str))
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let items_str: Option<String> = Deserialize::deserialize(deserializer)?;

        if items_str.is_none() || items_str.as_ref().unwrap().is_empty() {
            return Ok(None);
        }

        let items = items_str
            .unwrap()
            .split(ITEM_SEPARATOR)
            .map(|item| T::from_str(item))
            .collect::<Result<Vec<_>, T::Err>>()
            .map_err(|err| {
                serde::de::Error::custom(format!("Unable to parse space separated list {err:?}"))
            })?;

        Ok(Some(items))
    }
}
