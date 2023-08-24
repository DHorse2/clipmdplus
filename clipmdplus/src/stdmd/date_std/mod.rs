use std::fmt;
// use std::time::Duration;
use std::time::SystemTime;

// use chrono::Utc;
// use chrono::NaiveDateTime;
// use chrono::serde::ts_seconds_option;
// date_std mod.rs

use serde::Deserialize;
// use serde::de::Error;
// use serde::de::{self, Unexpected, Visitor};
// use serde::de::{MapAccess, SeqAccess};
use serde::Serialize;
// use serde::ser::SerializeStruct;
// use serde::forward_to_deserialize_any;
// use serde::forward_to_deserialize_any_helper;
// use serde_json::{Deserializer, Serializer, Value};

// !------------------------------------------------------------
// Date Stuff

// !------------------------------------------------------------
/// DataCreationTime is a (currently unused) module's whose purpose is questionable.
/// It is intended to handle non-standard date formats and serialization.
/// todo review DataCreationTime
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "data_creation_time")]
pub struct DataCreationTime {
    // #[serde(default)]
    // #[serde(with = "ts_seconds_option")]
    // SystemTime
    /// The data being stored. There are several type options.
    /// The first in this list was chosen.
    ///     pub datetimesystem: Option<SystemTime>,
    ///     pub datetimeutc: Option<chrono::NaiveDateTime>,
    ///     pub datetimeutc: Option<chrono::DateTime<Utc>>,
    ///     pub datetimeutc: DateTime<Utc>,
    ///     datetimelocal: chrono::DateTime<Local>,
    ///     datetimeutc: Type::DateTime,
    pub datetimesystem: Option<SystemTime>,
}
impl Eq for DataCreationTime {}
// impl std::ops::Deref for DataCreationTime {
//     type Target = Option<chrono::DateTime<Utc>>;

//     fn deref(&self) -> &Self::Target {
//         &self.datetimeutc
//     }
// }
impl DataCreationTime {
    /// creates a new DateTime set to now().
    pub fn new() -> Self {
        Self {
            datetimesystem: Some(SystemTime::now()),
            // datetimeutc: Some(Utc::now()),
        }
    }

    // Notes:
    // Source: StackExchange - Is there a Rust crate that implements the SQL date and interval types?
    // Chrono::NaiveDate::parse_from_str(s, fmt) mostly solved the first problem.
    // I don't have intervals yet, but I can do dates. I do have to specify the format of the date, e.g. "%Y-%m-%d" but it seems the only option to do so.
    // And to get an integer, I am using the timestamp() function. That gives me a signed integer, which I can then convert to unsigned.
    // https://dba.stackexchange.com/questions/302283/is-there-a-rust-crate-that-implements-the-sql-date-and-interval-types
}

impl Default for DataCreationTime {
    /// creates a new DateTime. Similarly to new() it is set to now().
    fn default() -> Self {
        DataCreationTime {
            // datetimeutc: DateTime::<Utc>::from_local(chrono::NaiveDateTime::new(), Utc)
            datetimesystem: Some(SystemTime::now()),
        }
    }
}

impl fmt::Display for DataCreationTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.datetimesystem)
    }
}
// ------------------
// impl Serialize for DataCreationTime {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         // 3 is the number of fields in the struct.
//         let mut state = serializer.serialize_struct("DataCreationTime", 1)?;
//         state.serialize_field("datetimeutc", &self)?;
//         // state.serialize_field("datetimeutc", &self.datetimeutc)?;
//         state.end()
//     }
// }
// ------------------
// -----------------------------Deserialize-------------------------------
// Deserialize student work
// impl<'de> Visitor<'de> for DataCreationTime {
//     // fn deserialize<V>(self, visitor: V) -> Result<V::Value, Self::Error>
//     forward_to_deserialize_any!{
//     struct
//     }
//     forward_to_deserialize_any_helper!{
//     identifier<'de,V>
//     }
//     forward_to_deserialize_any_helper!{
//     ignored_any<'de,V>
//     }
// }
//
// impl<'de> Deserialize<'de> for DataCreationTime {
// forward_to_deserialize_any! {
//     bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
//     bytes byte_buf option unit unit_struct newtype_struct seq tuple
//     tuple_struct map struct enum identifier ignored_any
// }
//
// fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     // impl<'de> Visitor<'de> for DataCreationTime {
//     type Value = String;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "a string containing at least {} bytes", 0)
//     }
//
//     fn visit_str<E>(self, s: &DataCreationTime) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         type Err = de::Error;
//         // convert s to: &DataCreationTime
//         if s.datetimeutc() >= 0 {
//             Ok(s.to_owned())
//         } else {
//             Err(de::Error::invalid_value(Unexpected::Str(s), &self))
//         }
//     }
// }
// const FIELDS: &'static [&'static str] = &["datetimeutc"];
//
// struct FieldVisitor; // Not implemented. Maybe for FormatTypes
// impl FieldVisitor {}
// impl<'de> Visitor<'de> for FieldVisitor {
//     type Value = Field;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("A valid UTC date is required. Not: {datetimeutc}")
//     }
//
//     fn visit_str<E>(self, value: &str) -> Result<Field, E>
//     where
//         E: de::Error,
//     {
// Mockup
// Ok(Field::new(String::from(value), Type::TEXT))
// Mockup
// if !true {
//     Err(de::Error::unknown_field(value, FIELDS))
// } else {
// Ok( Field::new(value.to_string(), Type::DATE))  // type::text // Mockup
// Ok(
// ? maybe ? datetimeutc <- ye
// This turns out to be wrong (below) because this is a Field Visitor not a Struct Visitor
// DataCreationTime {
// datetimeutc: datetimeutc::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor))
// datetimeutc: DateTime::parse_from_str(deserializer.deserialize_struct("DataCreationTime", FIELDS, FieldVisitor))
// deserializer.deserialize_struct("DataCreationTime", FIELDS, FieldVisitor)
// }
//
//
//     match value {
//         "datetimeutc " => {
//             // Ok(deserializer.deserialize_string(visitor)) // ;; .deserialize_struct("DataCreationTime", FIELDS, FieldVisitor))
//             // Ok(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor))
//             // Ok(&self.datetimeutc)
//             // Ok(DateTime::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor)))
//             Ok(Field::from(
//                 DateTime::parse_from_str(&self.deserialize_struct("datetimeutc", FIELDS, FieldVisitor).unwrap()).unwrap()
//                 // DateTime::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELDS, FieldVisitor)).unwrap()
//             ))
//         }, // th                            },
//         "DataCreationTime" => {
//             Ok(
//                 DateTime::parse_from_str(deserializer.deserialize_struct("DataCreationTime", FIELDS, FieldVisitor), "%c")
//             )
//         }, // the whole struct???
//         _ => Err(de::Error::unknown_field(value, FIELDS)),
//         // "datetimeutc " => Ok(DataCreationTime::datetimeutc),
//     }
// // )
//
// } // mockup
// match value {
//     "shouldbe datetimeutc " => Ok(DataCreationTime::datetimeutc),
//     "datetimeutc" => Ok(&value),
//     _ => Err(de::Error::unknown_field(value, FIELDS)),
// }
// }
// }
//
// DataCreationTime::new(datetimeutc: Value);
// let tmp: DataCreationTime =
// deserializer.deserialize_struct("DataCreationTime", FIELDS, FieldVisitor);
// // Ok(deserializer.deserialize_string(datetimeutc)) // ;; .deserialize_struct("DataCreationTime", FIELDS, FieldVisitor) )
// Ok(tmp)
//
// Ok(
// DataCreationTime {
//     datetimeutc: DateTime::parse_from_str(
// Deserialize::deserialize(deserializer) //.unwrap().to_string()
// Deserialize::deserialize_any(deserializer) //.unwrap().to_string()
// , "%c").unwrap().into(),
// }
// DataCreationTime { datetimeutc: DateTime::parse_from_str(deserializer.deserialize_any(deserializer).unwrap().to_string(), "%c").unwrap().into() }
//     // DataCreationTime { datetimeutc: DateTime::parse_from_str(deserializer.deserialize_struct("datetimeutc", FIELZDS, FieldVisitor), "%c") }
//     // DataCreationTime { datetimeutc: DateTime::parse_from_str(deserializer.deserialize_field("datetimeutc", FIELDS, FieldVisitor), "%c") }
// )
//
// struct DataCreationTimeVisitor;
//
// impl<'de> Visitor<'de> for DataCreationTimeVisitor {
//     type Value = DataCreationTime;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("A valid UTC date")
//     }
//
//     fn visit_string<V>(self, mut value: V) -> Result<DataCreationTime, V::Error>
//     where
//         V: SeqAccess<'de>,
//     {
//         let s: DataCreationTime = serde::Deserialize::deserialize(value)?
//         // Ok(DataCreationTime::from(desialzer.deserialize_string("DataCreationTime", FIELDS, DurationVisitor)))
//
//     }
//     // deserializer.deserialize_identifier(FieldVisitor)
//     // Deserialize::deserialize(deserializer)
//     // deserializer.deserialize_struct("DataCreationTime", FIELDS, DurationVisitor)
// }
//     }
// }
// !------------------------------------------------------------
