# Developer Notes

## Derive_more notes

use derive_more::{Add {...}, AsMut, AsRef, Display, From, FromStr, Into, IntoIterator, TryInto};

use Display-like, contains Display, Binary, Octal, LowerHex, UpperHex, LowerExp, UpperExp, Pointer

### Operators

These are traits that can be used for operator overloading.

- Index
- Deref
- Not-like, contains Not and Neg
- Add-like, contains Add, Sub, BitAnd, BitOr, BitXor
- Mul-like, contains Mul, Div, Rem, Shr and Shl
- Sum-like, contains Sum and Product
- IndexMut
- DerefMut
- AddAssign-like, contains AddAssign, SubAssign, BitAndAssign, BitOrAssign and BitXorAssign
- MulAssign-like, contains MulAssign, DivAssign, RemAssign, ShrAssign and ShlAssign

## Crates

### External Crates Evaluated (short list)

There were numerous serde crates skipped here.

- extern crate anyhow; (no - static types preferred)
- extern crate blob; (doesn't implement ToSql trait)
- extern crate derive_name;
- extern crate derive_more;
- extern crate egui;
- extern crate eframe; (for egui)
- extern crate serde;
- extern crate serde_derive;
- extern crate serde_enum_str; (buggy)
- extern crate serde_json;
- extern crate serde-error (no - uses anyhow::result)
- extern crate thiserror; (unit errors)

- extern crate clipmdplus
- extern crate clipmdplus_library
- extern crate clipmdplus_macro

### Crates Used

[macro_use].

- extern crate anyhow;            (no - static types preferred)
- extern crate blob;              (doesn't implement ToSql trait)
- extern crate derive_name;       (not working so far - trying it)
- extern crate derive_more;
- extern crate egui;
- extern crate eframe;            (for egui)
- extern crate serde;
- extern crate serde_derive;
- extern crate serde_enum_str;    (buggy)
- extern crate serde_json;
- extern crate serde-error        (no - uses anyhow::result)
- extern crate thiserror;         (unit errors only)

## Dependencies

[dependencies]

- blob = "0.3.0"
- chrono = {version = "0.4.23", features = ["serde"]}
- libc = "0.2.141"
- postgres = "0.19.4"

### http / json

- reqwest = {version = "0.11.18", features = ["json"]}

### Serialization

- serde = {version = "1.0.160", features = ["derive", "rc"]}
- serde-enum = "0.1.4"
- serde-enum-str = "0.3.2"
- serde_json = "1.0.96"

### System

- sled = "0.34.6"
- sys = "0.0.0"
- thiserror = "1.0.40"

### async

- lazy_static = "1.4.0"
- tokio = { version = "1.28.2", features = ["full"] }

### UI

- egui = { version = "0.22.0"}
        , features = ["provide_any"] }
        (enum_derive = "0" can't have payloads)
- eframe = "0.22.0"
- egui_web = "0.17.0"

### Proc Macro

- derive_more = "0.99.17"
- derive-name = "0.1.2"
- proc-macro2 = "1.0.60"
- syn = "2.0.18"
- quote = "1.0.28"

### Windows:    OS

- [dependencies.windows]
- version = "0.48"
- features = [
- "Data_Xml_Dom",
- "Win32_Foundation",
- "Win32_Security",
- "Win32_System_Threading",
- "Win32_UI_WindowsAndMessaging",
- ]
- [dependencies.windows-sys]
- version = "0.48"
- features = [
- "Win32_Foundation",
- "Win32_Security",
- "Win32_System_Threading",
- "Win32_UI_WindowsAndMessaging",
- ]

### Development

- [dev-dependencies]

- [maybe]
- Command-line argument parsing clap = "2.25.0"📖
- Global initialization lazy_static = "0.2.8"📖
- Random numbers rand = "0.3.15"
- Tar archives tar = "0.4.23" 📖
- Temporary directories tempdir = "0.3.5"
- Thread pool threadpool = "1.4.0"
- Configuration files toml = "0.4.2"
- URLs url = "1.5.1"
- Directory traversal walkdir = "1.0.7"

### [Unused]

- ansi_escapes = "0"
- ansi_escape = "0"
- local-ip-address = "0.5.1"
- whoami = "1.4.0"
- sys = "0"
- term_cursor = "0.2.1"
- mtermion = { path = "..\\mtermion_2.0.1" }
- mtermion = { path = "G:\\KBil23\\Code\\Vs\\Vs0\\mtermion_2.0.1" }
- extra = "0.1.0"

## Saved USE statements (pretty layout)

//
// use serde::{
//     Deserialize, Serialize,
// };
// use std::env;
// use std::env::consts::*;
// use std::error::Error;
use std::fmt;
// use std::fs::File;
//
// use std::io::prelude::*;
// use std::io::{
    // Error,
    // ErrorKind,
    // self,
    // BufReader,
    // Read,
    // Result,
    // Write,
// };
// use std::os::windows::prelude::*;
// use std::path::PathBuf;
// use std::process;
use std::str;
use crate::UiType;
// use std::thread;
//
// (see chrono below)
// use std::time::{
//     Duration,
//     Instant,
//     SystemTime
// };
//
// use crate::clip_form::egui::native:
// use crate::clip_form::egui::native::FormWindow;
// use crate::clip_form::egui::native::WindowOptions;
use crate::clip_form::egui::native::ClipFormEgui;

use crate::clip_util::ClipMeta;
// use crate::clip_util::ClipboardMeta;
use crate::clip_util::ProgControl;
//
// use crate::stdmd::hyperlink::HyperlinkMeta;
use crate::stdmd::hyperlink::HyperlinkList;
use crate::stdmd::range::Range;
// use crate::stdmd::types::UiType;
// use crate::stdmd::types::UiError;
// use crate::stdmd::types::UiResult;
// use crate::stdmd::types::WindowType;
// use crate::stdmd::types::WindowError;
// use crate::stdmd::types::WindowResult;
use crate::stdmd::sequence::Sequence;

pub mod egui;

## Cargo crates

// Windows:# OS
// # [dependencies.windows]
// # version = "0.48"
// # features = [
// #     "Data_Xml_Dom",
// #     "Win32_Foundation",
// #     "Win32_Security",
// #     "Win32_System_Threading",
// #     "Win32_UI_WindowsAndMessaging",
// # ]
// # [dependencies.windows-sys]
// # version = "0.48"
// # features = [
// #     "Win32_Foundation",
// #     "Win32_Security",
// #     "Win32_System_Threading",
// #     "Win32_UI_WindowsAndMessaging",
// # ]
// # Development:
// [dev-dependencies]

// # [maybe]
// # Command-line argument parsing  clap = "2.25.0"  📖 **
// # Global initialization  lazy_static = "0.2.8"  📖
// # Random numbers  rand = "0.3.15"  📖
// # Tar archives  tar = "0.4.23"  📖
// # Temporary directories  tempdir = "0.3.5"  📖
// # Thread pool  threadpool = "1.4.0"  📖
// # Configuration files  toml = "0.4.2"  📖
// # URLs  url = "1.5.1"  📖
// # Directory traversal  walkdir = "1.0.7"  📖

// # [Unused]
// # ansi_escapes = "0"
// # ansi_escape = "0"
// # local-ip-address = "0.5.1"
// # whoami = "1.4.0"
// # sys = "0"
// # term_cursor = "0.2.1"
// # mtermion = { path = "..\\mtermion_2.0.1" }
// # mtermion = { path = "G:\\KBil23\\Code\\Vs\\Vs0\\mtermion_2.0.1" }
// # extra = "0.1.0"

## xxx
