// stdmd app_error.rs
/// AppError
/// UiError
/// DbError
// !------------------------------------------------------------
use std::error::Error as StdError;
// use std::io::Error as StdIoError;
//
// extern crate serde;
use serde::Serialize;
use serde::Deserialize;
// use serde_json::Error as AppJsonError;
// use serde::de::Error as AppSerdeDeError;
// use serde::ser::Error as AppSerdeSerError;
// use serde::ser::Error;
// use serde::de::Error;
// use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
// use serde_enum::Deserialize_enum;
// use serde_enum::Serialize_enum;
// use serde_enum::ToString;
use std::fmt;
//
// use thiserror::Error as ErrorDerive;
//
// use crate::clipmdplus_macro::Name;
use crate::clipmdplus_macro::VariantName;
use crate::clipmdplus_macro::Deserialize_enum;
use crate::clipmdplus_macro::Serialize_enum;
//
// extern crate derive_more;
// use crate::derive_more::Display;
use crate::derive_more::From;
//
// !------------------------------------------------------------
// Error data structs and enums
include!(".\\app_error_data.rs");
// !------------------------------------------------------------
/// PhantomData for Errors.
pub struct PhantomError;
// !------------------------------------------------------------
/// App error is a simplified error type.
/// It might be returned to the App.
/// Under review. Might be discarded.
// #[derive(Debug, Deserialize, Serialize)]
#[derive(Debug, derive_more::Display, thiserror::Error, VariantName, Deserialize_enum, Serialize_enum,  /* */ )]
pub enum AppError {
    /// from the database.
    DbError,
    /// from the UI.
    UiError,
    /// Unknown source.
    Unknown,
}
// !------------------------------------------------------------
/// Errors received from GUI components or upon validation.
// #[derive(Debug, Deserialize, Serialize)] // ErrorDerive, 
#[derive(Debug, derive_more::Display, thiserror::Error, VariantName, Deserialize_enum, Serialize_enum,  /* */ )]
pub enum GuiError {
    /// Entry format invalid (ie like dates).
    InvalidInput,
    /// Unable to convert the value.
    InvalidConversion,
    /// The clipboard data is bad and can't be used.
    InvalidClipboardData,
    /// The error type, source or cause is unknown.
    Unknown,
}
// !------------------------------------------------------------
// DATABASE
// !------------------------------------------------------------
// This was an attempt to 
// #[derive(thiserror::Error, Deserialize, Serialize)]
// #[derive(thiserror::Error, Deserialize_enum_str, Serialize_enum_str)]
// todo #[derive(Debug, Deserialize, Serialize)] 
// todo thiserror::Error, 
// #[derive(Deserialize_enum, Serialize_enum,  /* */ )]
// #[derive(Debug, derive_more::Display, thiserror::Error, derive_name::VariantName, serde_enum::Deserialize_enum, serde_enum::Serialize_enum,  /* */ )]
// #[derive(Clone, derive_more::Display, thiserror::Error, derive_more::From, VariantName, serde::Deserialize, serde::Serialize,  /* */ )]
/// A custom database error.
#[derive(Clone, thiserror::Error, derive_more::From, VariantName, serde::Deserialize, serde::Serialize,  /* */ )]
pub enum DbError {
    /// #[error("database client error {DbErrorDataClient::Name}")]
    // Client(#[from] DbErrorDataClient),
    Client(DbErrorDataClient),

    /// #[error("deserialize error {DbErrorDataDe}")]
    Deserialize(DbErrorDataDe), 
    // Deserialize(#[from] DbErrorDataDe), 
    // Deserialize(#[from] Box<dyn SerdeDeError>), (a Trait) 
    // Deserialize(#[from] e<SerdeDeError>), 
    // serde::de::Error
    // Deserialize(#[from] String), 

    /// #[error(transparent)]
    /// StdIoError(#[from] std::io::Error),
    StdIoError(DbErrorDataIo),

    /// #[error("json error {JsonErrorData}")]
    Json(JsonErrorData),

    /// #[error("postgres SQL error {DbErrorDataPostgres}")]
    // #[error(transparent)]
    Postgres(DbErrorDataPostgres),
    // Postgres(#[from] Box<postgres::error::Error>),

    /// #[error("serialize error {DbErrorDataSer}")]
    Serialize(DbErrorDataSer),

    /// #[error("unknown error {DbErrorData}")]
    Unknown(DbErrorData),

    /// #[error("general std error {DbErrorDataStd}")]
    Std(DbErrorDataStd),

    /// #[error("phantom error")]
    PhantomError,
    // X = (()),
}
// Implement std::convert::From for AppError; from io::Error
impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        DbError::StdIoError(DbErrorDataIo(DbErrorData::new(error.to_string().as_str())))
        // {
            // message: error.to_string(),
            // kind: String::from("io"),
            // message: error.to_string(),
        // }
    }
}
// Implement std::convert::From for AppError; from io::Error
impl From<postgres::error::Error> for DbError {
    fn from(error: postgres::error::Error) -> Self {
        DbError::Postgres(DbErrorDataPostgres(DbErrorData::new(error.to_string().as_str())))
        // data: DbErrorData {
        //     data: error.to_string,
        //     // kind: String::from("io"),
        //     // message: error.to_string(),
        // };
        // DbError::IoError(data)
    }
}
// impl From<postgres::error::Error> for DbError {
//     fn from(error: postgres::error::Error) -> Self {
//         DbError::Postgres { DbErrorDataPostgres::new(error.as_str()) }
//     }
// }
// Implement std::fmt::Debug for DbError
impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }} [{}]", file!(), line!(), self) // programmer-facing output
    }
}
// Implement std::fmt::Display for DbError
// impl fmt::Display for DbError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "database error at ({}: {}) {}", file!(), line!(), self) // user-facing output
//     }
// }
// !------------------------------------------------------------
// #[derive(thiserror::Error, serde_enum::Deserialize_enum, serde_enum::Serialize_enum,  /* */ )]
// #[derive(thiserror::Error, Deserialize, Serialize)] 
// #[derive(thiserror::Error, derive_name::VariantName, serde_enum::Deserialize_enum, serde_enum::Serialize_enum,  /* */ )]
// #[derive(Clone, derive_more::Display, thiserror::Error, derive_more::From, VariantName, serde::Deserialize, serde::Serialize,  /* */ )]
/// A custom JsonError needed later.
#[derive(Clone, thiserror::Error, VariantName, serde::Deserialize, serde::Serialize,  /* */ )]
pub enum JsonError {
    /// A Json parsing error.
    #[error("parsing error {0:?}")]
    Parsing(String),
    /// Generic or unknown IO errors.
    #[error("io error {0:?}")]
    Io(String),
    /// Errors received from std::io.
    #[error("std::io error {0:?}")]
    IoError(DbErrorDataStd),
    /// Serde errors.
    #[error("serde json error {0:?}")]
    Serde(DbErrorData), // serde_json::Error
    // An error of unknown type or orgin.
    /// Unknown {#[from]  data: String },
    #[error("unknown error {0:?}")]
    Unknown(String),
}
// Implement std::fmt::Debug for JsonError
impl fmt::Debug for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
// Implement std::fmt::Display for JsonError
// impl fmt::Display for JsonError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "database error at ({}: {}) {}", file!(), line!(), self.0) // user-facing output
//     }
// }
// !------------------------------------------------------------

// !------------------------------------------------------------
// , Deserialize_enum_str, Serialize_enum_str
// #[derive(thiserror::Error, Deserialize, Serialize)]
// #[derive(thiserror::Error, Deserialize_enum_str, Serialize_enum_str)]
// pub enum DbError {
//     #[error("database client error {data}")]
//     Client {#[from] data: DbErrorDataClient},
//     #[error("deserialize error {data}")]
//     Deserialize {#[from] data: DbErrorDataDe}, 
//     // Deserialize(#[from] DbErrorDataDe), 
//     // Deserialize(#[from] Box<dyn SerdeDeError>), (a Trait) 
//     // Deserialize(#[from] e<SerdeDeError>), 
//     // serde::de::Error
//     // Deserialize(#[from] String), 
//     #[error(transparent)]
//     IoError (#[from] Box<io::Error>),
//     #[error("json error {data}")]
//     Json {#[from] data: JsonErrorData},
//     #[error("postgres SQL error {data}")]
//     Postgres {#[from] data: postgres::error::Error},
//     #[error("serialize error {data}")]
//     Serialize {#[from] data: DbErrorDataSer},
//     // Serialize(#[from] DbErrorDataSer),
//     // Serialize(#[from] Box<dyn SerdeSerError>), (a Trait)
//     // serde::ser::Error
//     // Serialize(#[from] String),
//     #[error("unknown error {data}")]
//     Unknown {#[from] data: DbErrorData},
//     #[error("general std error {data}")]
//     Std {#[from] data: DbErrorDataStd},
// }



// pub enum DbError {
//     #[error("database client error {data}")]
//     Client {#[from] data: DbErrorDataClient },

//     #[error("deserialize error {data}")]
//     Deserialize {#[from] data: DbErrorDataDe }, 
//     // Deserialize(#[from] DbErrorDataDe), 
//     // Deserialize(#[from] Box<dyn SerdeDeError>), (a Trait) 
//     // Deserialize(#[from] e<SerdeDeError>), 
//     // serde::de::Error
//     // Deserialize(#[from] String), 

//     #[error(transparent)]
//     StdIoError {#[from] data: DbErrorDataIo },
//     // StdIoError(#[from] std::io::Error),

//     #[error("json error {data}")]
//     Json {#[from] data: JsonErrorData },

//     // #[error("postgres SQL error {data}")]
//     #[error(transparent)]
//     Postgres {#[from] data: DbErrorDataPostgres },
//     // Postgres(#[from] Box<postgres::error::Error>),

//     #[error("serialize error {data}")]
//     Serialize {#[from] data: DbErrorDataSer },
//     // I tried everything :( 
//     // Serialize(#[from] DbErrorDataSer),
//     // Serialize(#[from] Box<dyn SerdeSerError>), (a Trait)
//     // serde::ser::Error
//     // Serialize(#[from] String),

//     #[error("unknown error {data}")]
//     Unknown {#[from] data: DbErrorData },

//     #[error("general std error {data}")]
//     Std {#[from] data: DbErrorDataStd },

//     #[error("phantom error")]
//     PhantomError,
//     // X = (()),
// }
