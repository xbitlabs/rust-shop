pub mod date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
pub mod long_format{
    use serde::{Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(number: &i64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = number.to_string();
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse()
            .map_err(serde::de::Error::custom)
    }
}
pub mod option_long_format{
    use std::num::ParseIntError;
    use anyhow::anyhow;
    use serde::{Deserialize, Deserializer, Serializer};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(number: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        if number.is_none() {
            let s = String::from("null");
            serializer.serialize_str(&s)
        }else {
            let s = number.unwrap().to_string();
            serializer.serialize_str(&s)
        }

    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "null" {
            return Ok(None)
        }
         let number:Result<i64,ParseIntError> = s.parse();
        if number.is_ok() {
            Ok(Some(number.unwrap()))
        }else {
            Err(serde::de::Error::custom("parse Option<i64> error"))
        }
    }
}
pub mod long_vec_format{
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::ser::SerializeTuple;

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(numbers: &Vec<i64>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(numbers.len())?;
        for e in numbers.iter() {
            seq.serialize_element(e.to_string().as_str())?;
        }
        seq.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        serde_json::from_str(s.as_str())
            .map_err(serde::de::Error::custom)
    }
}