// clip_util mod.rs
// Clipboard ClipUtil module (C# namespace)
//
// Early development
// #[cfg(debug_assertions)]
#![cfg_attr(debug_assertions, allow(dead_code, unused, unused_imports))]
//
// #[allow(dead_code, unused, unused_imports)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
//
use blob::Blob;
//
extern crate chrono;
use chrono::{DateTime, NaiveDateTime};
// use chrono::naive::NaiveDate;
use chrono::offset::Utc;
// use chrono::offset::FixedOffset;
use chrono::serde::ts_seconds_option;
//
extern crate derive_more;
use derive_more::{Add, Display, From, Into};

use postgres::{Client, NoTls};
use postgres::Column;
// use postgres::Error;
use postgres::row::Row;
use postgres::types::Date;
use postgres::types::Field;
use postgres::types::Type;
use postgres::types::ToSql;
//
// What is the default meaning of Error?
// use crate::stdmd::Error;
// use postgres::Error;
// use serde::ser::Error;
// use serde::de::Error;
// use serde_json::Error;
//
use serde::de::{MapAccess, SeqAccess};
use serde::de::{self, Unexpected, Visitor};
use serde::ser::SerializeStruct;
use serde::{
    forward_to_deserialize_any, forward_to_deserialize_any_helper, Deserialize, Serialize,
};
//
use serde_json::{Deserializer, Serializer, Value};
//
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;
// use std::time::Instant;
use std::time::SystemTime;
//
// use std::error::Error;
//
use crate::stdmd::{self, db_api};
use crate::stdmd::db_api::DbError;
use crate::stdmd::db_api::JsonError;
use crate::stdmd::db_api::DbType;
use crate::stdmd::db_api::DbApi;
use crate::stdmd::db_api::DbCrud;
use crate::stdmd::db_api::DbJson;
use crate::stdmd::date_std::DataCreationTime;
use crate::stdmd::sequence::Sequence;
use crate::stdmd::range::Range;
use crate::stdmd::hyperlink::*;

// use crate::stdmd::{
//     self, 
//     db_api::{
//         DbError,
//         JsonError,
//         DbType,
//         DbApi,
//         DbCrud,
//         DbJson
//     }
// };
use crate::stdmd::date_std::DataCreationTime;
use crate::stdmd::sequence::Sequence;
use crate::stdmd::range::Range;
use crate::stdmd::hyperlink::*;
// !------------------------------------------------------------
include!(".\\format.rs");
// !------------------------------------------------------------
include!(".\\i_data_object.rs");
// !------------------------------------------------------------
include!(".\\clip_meta.rs");
// !------------------------------------------------------------
// include!("..\\std\\mod.rs");
// !------------------------------------------------------------
// include!("..\\std\\date_std\\mod.rs");
// !------------------------------------------------------------
// include!("..\\std\\sequence.rs");
// !------------------------------------------------------------
// include!("..\\std\\range.rs");
// !------------------------------------------------------------
// include!("..\\std\\hyperlink\\mod.rs");
// !------------------------------------------------------------
// Window
// include!("..\\clip_form\\mod.rs");
// !------------------------------------------------------------
pub struct DoSerial {} // DoSerial : IXmlSerializable
impl DoSerial {
    // todo!
}
// !------------------------------------------------------------
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgControl {
    pub db_type: DbType,
    pub id_key_current: u32,
    pub sequence_number: u32,
    pub clip_count: u32,
    date_time_offset: DataCreationTime,
}
impl ProgControl {
    // todo!
}
impl Default for ProgControl {
    fn default() -> Self {
        ProgControl {
            db_type: stdmd::db_api::DbType::Postresql,
            id_key_current: u32::default(),
            sequence_number: u32::default(),
            clip_count: u32::default(),
            date_time_offset: DataCreationTime::default(),
        } 
            // ! todo
    }
}
// !------------------------------------------------------------
/// <summary>
/// Clipboard Formats enumeration
/// </summary>

// !------------------------------------------------------------
// Windows stuff
// !------------------------------------------------------------
// Postresql stuff
// !------------------------------------------------------------

// !------------------------------------------------------------
#[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
pub enum Message {
    Request,
    Response,
}

// !------------------------------------------------------------
// More Notes:
// Mdm CSharp C# Dependencies:
// using System.Runtime;
// using System.Runtime.Serialization;
// using System.Runtime.Serialization.Formatters.Binary;
// //using System.Runtime.Serialization.Formatters.Binary;
// using System.Text;
// //
// // add shell32.dll reference
// // or COM Microsoft Shell Controls and Automation
// using Shell32;
// using SHDocVw;
//
// using System.Xml;
// using System.Xml.Linq;
// using System.Xml.Serialization;
// //using System.Runtime.Serialization.XmlObjectSerializer;
// //using System.Runtime.Serialization.DataContractSerializer;
//
// using Mdm;
// using Mdm.Oss;
// using Mdm.Oss.Decl;
// using Mdm.Oss.File;
// using Mdm.Oss.Console;
// using Mdm.Oss.IeUtil;
// using Mdm.Oss.WinUtil;
// using Mdm.Oss.WinUtil.Types;
// using Mdm.World;
// using Mdm.World.Temporal;
// using Mdm.Oss.Sys;
// using Mdm.Oss.Sys.Binder;
//
// using System.Xml.Schema;
//
// // using Mdm.Oss.ClipUtil;
// // using Mdm.Oss.ClipUtil.Windows;
// !------------------------------------------------------------
