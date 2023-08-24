// stdmd.rs standard functions

/// A non-standard date time object
/// that has future use.
pub mod date_std; // romove?
pub use self::date_std::*;
/// Internet, file, folder, email, phone and document links.
pub mod hyperlink;
pub use self::hyperlink::*;
/// Simple database traits (C# friendly)
#[macro_use]
pub mod db_api;
pub use self::db_api::*;
/// types handling library.
#[macro_use]
pub mod types;
pub use self::types::*;
/// a learning task. 
/// todo Over defined errors are all over the place.
pub mod app_error;
pub use self::app_error::*;
