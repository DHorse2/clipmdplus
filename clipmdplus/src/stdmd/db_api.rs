// db_api.rs
//
use derive_more::Display;
use derive_more::From;
use derive_name::Name;
use derive_name::VariantName;
//
//
use serde_enum::Deserialize_enum;
use serde_enum::Serialize_enum;
use serde_enum::ToString;

// use enum_name_derive::EnumName;
// use serde_enum_str::Deserialize_enum_str;
// use serde_enum_str::Serialize_enum_str;
//
use serde::Serialize;
use serde::Deserialize;
// use serde_json::Error as JsonError;
// use serde::de::Error as SerdeDeError;
// use serde::ser::Error as SerdeSerError;
// use serde::ser::Error;
// use serde::de::Error;

// ! serialization of enums and complex types  --------------------------
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

// ! Types ------------------------------------------------------------
// DbType
// DbApi
// DbCrud
// type DbError = stdmd::db_api::DbError;
// type JsonError = serde_json::Error; // todo started api
// pub type DbClient = postgres::Client; // todo defaults 
// pub type DbClientError = postgres::Error;
// if let Err(fmt::Error)

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
// !------------------------------------------------------------
// include!(".\\db_error.rs");
// !------------------------------------------------------------
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
    // #[serde(rename = "Json")]
    // #[serde(rename = "Postgres")]
    // #[serde(other)]
// Serialize_enum_strm, Deserialize_enum_str, )]
// #[derive(VariantName)]
#[derive(Clone, derive_more::From, derive_name::VariantName, serde::Deserialize, serde::Serialize,  /* */ )]
// #[serde(rename_all = "snake_case")]
// #[derive(Clone, derive_more::From, derive_name::VariantName, serde_enum::ToString, serde_enum::Deserialize_enum, serde_enum::Serialize_enum,  /* */ )]
// , Serialize_enum_str, Deserialize_enum_str, )]
pub enum DbType {
    // #[serde(rename = "Json")]
    Json,
    // #[serde(rename = "Postgres")]
    Postgres,
    // #[serde(other)]
    Other(String),
    // #[serde(rename = "None")]
    None
}
impl DbType {
    fn as_str(&self) -> &str { &self.enum_name()}
}
// !------------------------------------------------------------
// <DbClient> <DbRow> DbApi DbCrud
pub trait DbApi {
    type DbType;
    type DbError;
    type DbClient;
    type DbClientError;

    fn db_connect() -> Result<Self::DbClient, Self::DbError>; // dyn postgres::Error
    fn db_disconnect() -> Result<bool, Self::DbError>; // dyn postgres::Error

    fn db_execute(&self, client: Self::DbClient, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Self::DbClientError>;

    // fn db_executeB(&self, client: Self::DbClient, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Self::DbClientError>
    // todo NOTE: uses dyn postgres::types::ToSql + Sync
    // How do you use an dynamic type's function?
    // {
    //     Self::client.execute( // <<<< fails
    //         query,
    //         params
    //     )?
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
    type JsonError;
    type JsonResult;
    fn to_json(&self) -> Self::JsonResult;
    fn from_json(json: &str) -> Self;
    fn load_json(mut file_path: &mut str) -> Self 
    where
        Self: Default,
    {
        if file_path.is_empty() {
            file_path = std::string::String::from("ClipboardData.txt".as_bytes_mut());
            // let mut file_path: &mut str = String::From("ClipboardData.txt");
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
