// db_api.rs

// use derive_more::From;
// use serde::ser::Error;
// use serde::de::Error;
use serde::Serialize;
use serde::Deserialize;
use serde::de::Error as SerdeDeError;
use serde::ser::Error as SerdeSerError;
// use serde::{
    // forward_to_deserialize_any, forward_to_deserialize_any_helper, 
    // Deserialize, Serialize,
// };
// use serde::de::{MapAccess, SeqAccess};
// use serde::de::{self, Unexpected, Visitor};
// use serde::ser::SerializeStruct;
//
// use serde_json::Deserializer;
// use serde_json::Serializer;
// use serde_json::Value;
//
use std::fmt;
use std::io;
use thiserror::Error;
//
// !------------------------------------------------------------
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DbType {
    Json,
    Postresql,
    None
}
// impl Clone for DbType {
//     fn clone(&self) -> DbType {
//         match self {
//             Json => Json,
//             Postresql => Postresql,
//             None => None,
//         }
//     }
// }
// #[serde(untagged)]
    // #[serde(rename = "Json")]
    // #[serde(rename = "Postresql")]
//
    // // This is what #[derive(Serialize)] would generate.
// impl Serialize for DbType {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut s = serializer.serialize_any(&self);
//         Ok(S::Ok)
//     }
// }
// !------------------------------------------------------------
//, Deserialize, Serialize)]
#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("deserialize error {0:?}")]
    Deserialize(#[from] Box<dyn SerdeDeError>), 
    // Deserialize(#[from] e<SerdeDeError>), 
    // serde::de::Error
    // Deserialize(#[from] String), 
    #[error(transparent)]
    IoError(#[from] Box<io::Error>),
    #[error("json error {0:?}")]
    Json(#[from] JsonError),
    #[error("postgres SQL error {0:?}")]
    Postresql(#[from] postgres::error::Error),
    #[error("serialize error {0:?}")]
    Serialize(#[from] Box<dyn SerdeSerError>),
    // serde::ser::Error
    // Serialize(#[from] String),
    #[error("unknown error {0:?}")]
    Unknown(#[from] String),
}
// Implement std::fmt::Debug for DbError
impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }} [{}]", file!(), line!(), self.0) // programmer-facing output
    }
}
// Implement std::fmt::Display for DbError
// impl fmt::Display for DbError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "database error at ({}: {}) {}", file!(), line!(), self.0) // user-facing output
//     }
// }
// !------------------------------------------------------------
#[derive(thiserror::Error, Deserialize, Serialize)]
pub enum JsonError {
    #[error("parsing error {0:?}")]
    Parsing(String),
    #[error("io error {0:?}")]
    Io(String),
    #[error(transparent)]
    IoError(#[from] io::Error),    
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
// <DbClient> <DbRow> DbApi DbCrud
pub trait DbApi {
    // type DbClient = postgres::Client;
    type DbClient;
    type DbError;

    fn db_connect() -> Result<Self::DbClient, Self::DbError>; // dyn postgres::Error
    fn db_disconnect() -> Result<bool, Self::DbError>; // dyn postgres::Error

    fn db_execute(&self, client: Self::DbClient, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Self::DbError>;
    // {
        // client.execute(
        //     query,
        //     params
        // )?
    // }
    fn db_exists(&self, client: Self::DbClient) -> Result<bool, Self::DbError>;
}
// !------------------------------------------------------------
// CRUD
pub trait DbCrud : DbApi {
    type DbRow;
    fn db_row_insert(&self, client: Self::DbClient) -> Result<u64, Self::DbError>;
    fn db_row_delete(&self, client: Self::DbClient) -> Result<u64, Self::DbError>;
    fn db_row_update(&self, client: Self::DbClient) -> Result<u64, Self::DbError>;
    fn db_row_exists(&self, client: Self::DbClient) -> Result<bool, Self::DbError>;

    fn db_row_get(row: Self::DbRow) -> Result<u64, Self::DbError>;
    fn from__row(row: Self::DbRow) -> Self;
}

pub trait DbJson {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Self;
    fn load_json(file_path: &str) -> Self 
    where
        Self: Default,
    {
        if file_path.is_empty() {
            file_path = "ClipboardData.txt";
        }
        let clip_json = std::fs::read_to_string(file_path).unwrap();
        if clip_json.is_empty() {
            return Self::default()
         }
        Self::default()
    }
}