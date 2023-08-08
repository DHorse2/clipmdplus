// stdmd app_error_data.rs
// no use (include)
// !------------------------------------------------------------
#[derive(Debug, derive_name::Name, serde::Deserialize, Serialize)]
pub struct DbErrorData {
     message: String,
     file: Option<String>,
     line: Option<u32>,
     column: Option<u32>,
     stack: Option<Vec<String>>
}
impl DbErrorData{
    pub fn new(message: &str) -> Self {
        DbErrorData {
            message: message,
            file: None,
            line: None,
            column: None,
            stack: None,
        }
    }
}
impl StdError for DbErrorData {}
impl fmt::Display for DbErrorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "database error {}", self.message)
    }
}
// !------------------------------------------------------------
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataIo { data: DbErrorData }
impl StdError for DbErrorDataIo {}
impl fmt::Display for DbErrorDataIo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "system io error {}", self)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataPostgres ( DbErrorData );
impl StdError for DbErrorDataPostgres {}
impl fmt::Display for DbErrorDataPostgres {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "general std error {}", self)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataStd { data: DbErrorData }
impl StdError for DbErrorDataStd {}
impl fmt::Display for DbErrorDataStd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "general std error {}", self)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataClient { data: DbErrorData }
impl StdError for DbErrorDataClient {}
impl fmt::Display for DbErrorDataClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "general std error {}", self)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonErrorData { data: DbErrorData }
impl StdError for JsonErrorData {}
impl fmt::Display for JsonErrorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "general std error {}", self)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataSer { data: DbErrorData }
impl StdError for DbErrorDataSer {}
impl fmt::Display for DbErrorDataSer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "serialization error {}", self.data)
    }
}
// !------------------------------------------------------------
#[derive(Debug, Deserialize, Serialize)]
pub struct DbErrorDataDe { data: DbErrorData }
impl StdError for DbErrorDataDe {}
impl fmt::Display for DbErrorDataDe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "deserialization error {}", self)
    }
}
// !------------------------------------------------------------
