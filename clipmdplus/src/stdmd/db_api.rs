// db_api.rs
#![allow(dead_code, unused_imports)] 
/// Db_api Types:
/// DbType
/// DbApi
/// DbCrud
/// type DbError = stdmd::db_api::DbError;
/// type JsonError = serde_json::Error; // todo started api
/// pub type DbClient = postgres::Client; // todo defaults 
/// pub type DbClientError = postgres::Error;
/// 

// use self::*;

extern crate serde;
use serde::Serialize;
use serde::Deserialize;

extern crate derive_more;
use derive_more::From;

extern crate clipmdplus_macro;
use clipmdplus_macro::Name;
use clipmdplus_macro::VariantName;

/// ! DbType -------------------------------------------------
/// DbType enumerates the chosen database to use
// #[derive(Clone, Debug, VariantName, serde::Deserialize, serde::Serialize,  /* */ )] // derive_more::From, 
// #[serde(rename_all = "snake_case")]
// #[derive(Clone, derive_more::From, derive_name::VariantName, serde_enum::ToString, serde_enum::Deserialize_enum, serde_enum::Serialize_enum,  /* */ )]
// , Serialize_enum_str, Deserialize_enum_str, )]
// , From
#[derive(Clone, Debug, From, Eq, Hash, Name, Ord, PartialEq, PartialOrd, VariantName, Deserialize, Serialize)]
pub enum DbType {
    /// Json.
    // #[serde(rename = "Json")]
    Json,
    /// Postgres.
    // #[serde(rename = "Postgres")]
    Postgres,
    /// Other (from client).
    // #[serde(other)]
    Other(String),
    /// InMemory is a temporary history.
    // #[serde(rename = "InMemory")]
    InMemory,
    /// Last would have no history.
    /// The clip output processing would be available.
    // #[serde(rename = "Last")]
    Last,
    /// None (save as Last). No history.
    /// todo That creates confusion. Pick one or the other.
    // #[serde(rename = "None")]
    None
}

impl DbType {
    fn as_str(&self) -> &str { crate::DbType::name() } // &self.name() // .enum_name()}
}
/// ! DpApi -------------------------------------------------------
/// DbClient - DbRow - DbApi - DbCrud
/// The DbApi trait defines a simplified interface for database access.
/// It was created to mirror idiomatic C# as idomatic RUST.
/// Specifically an existing C# app is rewritten here in Rust.
/// It's somewhat intuitive where your database uses has a Type and Client.
/// Custom error handling is supplied via DbError and DbClientError.
pub trait DbApi {
    /// The database type.
    type DbType;
    /// The error return by the implementation.
    type DbError;
    /// The database client used for accessing the DB.
    type DbClient;
    /// The database client error if it is used.
    type DbClientError;

    /// Connect to the database.
    fn db_connect() -> Result<Self::DbClient, Self::DbError>; // dyn postgres::Error
    /// Disconnect from the database.
    fn db_disconnect(client: Self::DbClient) -> Result<bool, Self::DbError>; // dyn postgres::Error
    /// Execute a database command.
    fn db_execute(&self, client: Self::DbClient, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Self::DbClientError>;
    /// Check to see if the database exists.
    fn db_exists(&self, client: Self::DbClient) -> Result<bool, Self::DbError>;
}
/// ! DbCrud ------------------------------------------------------
// CRUD
pub trait DbCrud : DbApi {
    // The database type.
    // type DbType;
    /// The error return by the implementation.
    type DbError;
    /// The database client used for accessing the DB.
    type DbClient;
    // The database client error if it is used.
    // type DbClientError;
    /// A database client row (record).
    type DbRow;
    /// Insert a row into the database.
    fn db_row_insert(&self, client: <Self as DbCrud>::DbClient) -> Result<u64, <Self as DbCrud>::DbError>;
    /// Delete a row in the database.
    fn db_row_delete(&self, client: <Self as DbCrud>::DbClient) -> Result<u64, <Self as DbCrud>::DbError>;
    /// Update an existing row in the database.
    fn db_row_update(&self, client: <Self as DbCrud>::DbClient) -> Result<u64, <Self as DbCrud>::DbError>;
    /// Check if the database row already exists.
    fn db_row_exists(&self, client: <Self as DbCrud>::DbClient) -> Result<bool, <Self as DbCrud>::DbError>;
    /// Get the database row.
    fn db_row_get(row: Self::DbRow) -> Result<u64, <Self as DbCrud>::DbError>;
    /// Convert the database row to Self.
    fn from_row(row: Self::DbRow) -> Self;
}

/// ! bJson ------------------------------------------------------
pub trait DbJson {
    /// The error returned by Json processing.
    type JsonError;
    /// The result returned by Json processing.
    type JsonResult;
    /// Convert self to Json format.
    fn to_json(&self) -> Self::JsonResult;
    /// Load self from Json formatted data.
    fn from_json(json: &str) -> Self;
    // fn load_json(file_path: &mut String) -> Self 
    // fn load_json(mut file_path: &mut String) -> Self 
    /// Get the Json data from disk and load Self.
    fn load_json(mut file_path: String) -> Self 
    where
        Self: Default,
    {
        if file_path.is_empty() {
            // file_path = &mut String::from("ClipboardData.txt").clone(); // &mut 
            file_path = String::from("ClipboardData.txt").clone(); // &mut 
        }
        let clip_json = std::fs::read_to_string(file_path).unwrap();
        if clip_json.is_empty() {
            return Self::default()
         }
        Self::default()
    }
}
// ! DbType unused traits ---------------------------------------------
// impl Clone for DbType {
//     fn clone(&self) -> DbType {
//         match self {
//             Json => Json,
//             Postgres => Postgres,
//             None => None,
//         }
//     }
// }
// #[serde(untagged)]
    // #[serde(rename = "Json")]
    // #[serde(rename = "Postgres")]
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
// CRATE NOTES
// if let Err(fmt::Error)
//
// use derive_more::Display;
// use derive_more::From;
//
// ! serialization solutions evaluated --------------------------
// use serde_enum::Deserialize_enum;
// use serde_enum::Serialize_enum;
// use serde_enum::ToString;
//
// use enum_name_derive::EnumName;
// use serde_enum_str::Deserialize_enum_str;
// use serde_enum_str::Serialize_enum_str;
//
// use serde::Serialize;
// use serde::Deserialize;
//
// use serde_json::Error as JsonError;
// use serde::de::Error as SerdeDeError;
// use serde::ser::Error as SerdeSerError;
// use serde::ser::Error;
// use serde::de::Error;
//
// ! serialization notes of enums and complex types  --------------------------
// Note: problem with serialization of enums. 
// Specifically non-Unit variants 20230805
// use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str}; (errors)
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
// use std::fmt;
// use std::io;
// use std::io::Error;
// use thiserror::Error;
// ! evaluate serde --------------------------------------------------------
// todo - evaluate serde
// - Persist std::io, serde, serde_json and custom errors.
// - Check out serde(with) usage for this.
// #[serde(rename = "name")]
// #[serde(skip)] 
// #[serde(skip_serializing)]
// #[serde(skip_deserializing)]
// #[serde(serialize_with = "path")] 
// #[serde(deserialize_with = "path")]
// #[serde(with = "module")]
// #[serde(default)]
// #[serde(flatten)]
// ! serde notes ------------------------------------------------
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
    // #[serde(rename = "Json")]
    // #[serde(rename = "Postgres")]
    // #[serde(other)]
// Serialize_enum_strm, Deserialize_enum_str, )]
// ! ClipMdPlus Macros -------------------------------------------------
// #[derive(VariantName)]
// #[macro_use]
// extern crate clipmdplus_macro;
// pub use clipmdplus_macro::*;
// (derive_name)
// use clipmdplus_macro::Name;
// use clipmdplus_macro::VariantName;
// (serde_enum)
// use clipmdplus_macro::Deserialize_enum;
// use clipmdplus_macro::Serialize_enum;
// use clipmdplus_macro::ToString;
//
// use super::clipmdplus_macro::Name;
// use super::clipmdplus_macro::VariantName;
