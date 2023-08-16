// clip_meta ClipboardMeta
// #[allow(dead_code, unused, unused_imports)] // object creation (pre debug)
// use std::fs::File;
// use std::fs::read_to_string;
// use std::io::prelude::*;
// use std::collections::HashMap;
// use crate::clip_util::ClipMeta;
// use crate::clip_util::ClipboardMeta;
// use crate::clip_util::ProgControl;
// use crate::stdmd::date_std::DataCreationTime;
// use crate::stdmd::range::Range;
// use crate::stdmd::sequence::Sequence;
// // use crate::stdmd::hyperlink::HyperlinkMeta;
// use crate::stdmd::hyperlink::HyperlinkList;
// use super::IDataObject;
// use super::
// use crate::clip_util::IDataObject;
// !------------------------------------------------------------
// ClipboardMeta
// ! NOT IMPLEMENTED

#[derive(Clone, Debug, From, Eq, Hash, Name, Ord, PartialEq, PartialOrd, Deserialize, Serialize, /* ... */)]
pub struct ClipboardMeta {
    // todo!(); The clipboard metadata and history
    // Permissions
    // App Permissions
    // Persistence settings
    // Sync Settings
    History: ClipboardHistory
}
#[derive(Clone, Debug, PartialEq, Eq, Hash, Name, Ord, PartialOrd, Deserialize, Serialize, /* ... */)]
pub struct ClipboardHistory {
    // ClipboardHistory Vector
    /// History Vector
    /// More performant for immutable data.
    /// todo will need interior mutability?
    // History: Arc<[ClipMeta]>
    History: Vec<ClipMeta>
}
// !------------------------------------------------------------
// ClipMeta
// PartialEq, 
// #[derive(Clone, Debug, Eq, serde::Deserialize, serde::Serialize)]
// #[serde(rename_all = "PascalCase")] // NO (?maybe?)
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, Hash, Name, Ord, PartialOrd, Deserialize, Serialize, /* ... */)]
pub struct ClipMeta {
    /// Unique auto-generated key (compliant)
    /// (This would have a getter/setter and not be public)
    pub id_key: u32,
    /// System Unique ID
    // Notes:
    /// GUID and MACs are all possible options
    /// it was previously not implemented.
    /// (Also, this would have a getter/setter and not be public)
    pub id_system: u32,
    /// System time as UTC
    pub data_creation_time: Option<SystemTime>,
    // pub data_creation_time: Option<DataCreationTime>,
    /// Updated
    pub data_processed: bool,
    /// Synchronization
    pub data_synced: Option<SystemTime>,
    /// Order of clip history
    pub sequence_number: u32,
    /// Clipboard data type
    pub data_type: String,
    /// Internal (this) clipboard data
    /// This is an object.
    /// In rust an : Arc<Box<dyn Any>>,
    pub clip_data: Option<String>, // mocked without deserializer
    /// Clipboard data object
    /// this is the OS object of Format Type
    /// in windows a Forms.DataObject
    // pub clip_i_data: Option<IDataObject>,
    // pub clip_i_data: Option<Blob>,
    pub clip_i_data: Option<Vec<u8>>,
}
// !------------------------------------------------------------
// id_key, id_system, data_creation_time, data_processed, data_synced, sequence_number, data_type,
// clip_data, clip_i_data, 
//
//id_key
//id_system
//data_creation_time
//data_processed
//data_synced
//sequence_number
//data_type
//clip_data
//clip_i_data
// !------------------------------------------------------------
impl ClipMeta {
    /// New: Default and T
    // fn new() -> Self {
    //     Default::default()
    // }
    pub fn new(t: ClipMeta) -> Self {
        t
    }
    // pub fn as_option(&self) -> Option<Self> {
    // todo experimental return self (ref?) as Option
    //     Option::Some(Self)
    // }
}
impl Default for ClipMeta {
    fn default() -> Self {
        ClipMeta {
            id_key: u32::default(),
            id_system: u32::default(),
            data_creation_time: Option::Some(SystemTime::now()),
            // data_creation_time: Option::Some(DataCreationTime::default()),
            data_processed: false,
            data_synced: None,
            sequence_number: u32::default(),
            data_type: String::new(),
            clip_data: None,
            clip_i_data: None,
        }
    }
}
impl fmt::Display for ClipMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "Clipboard data {} {} {:?}", self.id_key, self.data1, self)
        write!(f, "Clipboard data {} {} {:#?}", self.id_key, self.data_type, self.clip_data)
    }
}
// impl Eq for ClipMeta {}
impl PartialEq for ClipMeta {
    fn eq(&self, other: &Self) -> bool {
        self.id_key == other.id_key
    }
}
//
// !------------------------------------------------------------
impl DbApi for ClipMeta {
    type DbType = DbType;
    // use postgres::error::DbError
    // type DbError = stdmd::db_api::DbError;
    type DbError = postgres::Error;
    type DbClient = postgres::Client;
    type DbClientError = postgres::Error;
    //
    // database
    fn db_connect() -> Result<Self::DbClient, Self::DbError> {
        // let mut client = 
        postgres::Client::connect(
            "postgresql://postgres:postgres@localhost/library", 
            postgres::NoTls
        )
        // client?
        // Ok(client)
    }
    fn db_disconnect(mut client: Self::DbClient) -> Result<bool, Self::DbError> { // dyn postgres::Error
        Ok(true)
    }
    fn db_execute(&self, mut client: Self::DbClient, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, Self::DbClientError> {
            let results = client.execute(
                query,
                params
            )?;
            Ok(results)
    }
    fn db_exists(&self, mut client: Self::DbClient) -> Result<bool, Self::DbError> { // dyn postgres::Error
        Ok(true)
    }    
}
// !------------------------------------------------------------
impl DbCrud for ClipMeta {
    // type DbClient = postgres::Client;
    // type DbError = postgres::Error;
    type DbRow = postgres::row::Row;
    //
    fn db_row_insert(&self, mut client: Self::DbClient) -> Result<u64, Self::DbError> {
        let results = client.execute(
            "INSERT INTO clip_data (id_key, id_system, data_creation_time, data_processed, data_synced, sequence_number, data_type, clip_data, clip_i_data) VALUES VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &self.id_key,
                &self.id_system,
                &self.id_key,
                &self.id_system,
                &self.data_creation_time,
                &self.data_processed,
                &self.data_synced,
                &self.sequence_number,
                &self.data_type,
                &self.clip_data,
                &self.clip_i_data
            ],
        );
        // NOTE: clip_i_data is an independent IDataObject
        //
        // Ok(results)
        results
    }
    fn db_row_delete(&self, mut client: Self::DbClient) -> Result<u64, Self::DbError> {
        let results = client.execute(
            "INSERT INTO clip_data (id_key, id_system, data_creation_time, data_processed, data_synced, sequence_number, data_type, clip_data, clip_i_data) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &self.id_key,
                &self.id_system,
                &self.data_creation_time,
                &self.data_processed,
                &self.data_synced,
                &self.sequence_number,
                &self.data_type,
                &self.clip_data,
                &self.clip_i_data
            ],
        )?;
        Ok(results)
    }
    fn db_row_get(row: Self::DbRow) -> Result<u64, Self::DbError> {
        let clip_meta = ClipMeta {
            id_key: row.get("id_key"),
            id_system: row.get("id_system"),
            data_creation_time: row.get("data_creation_time"),
            data_processed: row.get("data_processed"),
            data_synced: row.get("data_synced"),
            sequence_number: row.get("sequence_number"),
            data_type: row.get("data_type"),
            clip_data: row.get("clip_data"),
            clip_i_data: row.get("clip_i_data"),
        };
        Ok(1)
    }
    fn from__row(row: Self::DbRow) -> Self {
        // db_row_get(row);
        ClipMeta {
            id_key:             row.get(0),
            id_system:          row.get(1),
            data_creation_time: row.get(2),
            data_processed:     row.get(3),
            data_synced:        row.get(4),
            sequence_number:    row.get(5),
            data_type:          row.get(6),
            clip_data:          row.get(7),
            clip_i_data:        row.get(8),
        }
    }

    fn db_row_update(&self, mut client: Self::DbClient) -> Result<u64, Self::DbError> {
        // todo!() define db row update
        Ok(1)
    }

    fn db_row_exists(&self, mut client: Self::DbClient) -> Result<bool, Self::DbError> {
        // todo!() define db row exists
        Ok(true)
    }
}
// !------------------------------------------------------------
impl DbJson for ClipMeta {
    // type JsonError = serde_json::Error;
    type JsonError = serde_json::Error;
    type JsonResult = serde_json::Result<String>;
    fn to_json(&self) -> Self::JsonResult {
        serde_json::to_string(&self)
    }
    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }
    fn load_json(file_path: &mut String) -> Self {
        if file_path.is_empty() {
            let file_path = "ClipboardData.txt";
        }
        let clip_json = std::fs::read_to_string(file_path).unwrap();
        if clip_json.is_empty() {
            return ClipMeta::default();
        }
        ClipMeta::from_json(clip_json.as_str())        
    }
}
// END OF CODE
// !------------------------------------------------------------
// Notes: ClipMeta
// / from: {
// public ClipMeta() { }
// / public ClipMeta(ClipMeta t) { this.Value = t; }
// / }

// / Getters and Setters:
// / idiomatic RUST issue.
// / Getters and Setters are neither idiomatic nor neccessay.
// / However when porting from DotNet C# they might be retained
// / for greater accuracy in code audit and software based conversion.
// /
// / In this specific case we see in the example below that these are genereated by DotNet
// / using syntax sugar. They are a #[derive] or Trait.
// / CSharp:
// / public ClipMeta Value { get; set; }
// / end.

        // Notes:
        // Call the native GetSystemTime method
        // with the defined structure.
        // DateTimeNow = chrono::DateTime();
        // Win32TimeDef.SYSTEMTIME DateTimeNow = new Win32TimeDef.SYSTEMTIME();
        // Win32TimeDef.GetSystemTime(DateTimeNow);
            // Notes:
                /*
                idKey = 0;
                // Call the native GetSystemTime method
                // with the defined structure.
                Win32TimeDef.SYSTEMTIME DateTimeNow = new Win32TimeDef.SYSTEMTIME();
                Win32TimeDef.GetSystemTime(DateTimeNow);
                // Load Current Time
                dataCreationTime = new DateTimeOffset(
                    DateTimeNow.wYear,
                    DateTimeNow.wMonth,
                    // DateTimeNow.DayOfWeek,
                    DateTimeNow.wDay,
                    DateTimeNow.wHour,
                    DateTimeNow.wMinute,
                    DateTimeNow.wSecond,
                    DateTimeNow.wMilliseconds,
                    CultureInfo.CurrentCulture.Calendar,
                    new TimeSpan(0)
                    );
                dataProcessed = false;
                sequenceNumber = 0;
                data1 = "";
                cData = ""; new Object();
                ciData = new DataObject();
                */
