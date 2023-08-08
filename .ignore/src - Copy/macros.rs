
// use std::net;
// use std::io::prelude::*;
// use std::fmt;

// !------------------------------------------------------------
// #[macro_use]
//  pub mod serde_enum_local;
//  pub use self::serde_enum_local::*;
//  include!(".\\macros\\serde_enum_local.rs");
// !------------------------------------------------------------

// !------------------------------------------------------------
#[macro_use]
 pub mod derive_name;
 pub use self::derive_name::*;
//  include!(".\\macros\\derive_name.rs");
// !------------------------------------------------------------

// From termion Create a CSI-introduced sequence.
#[allow(unused_macros)]
macro_rules! msequence {
    ($( $l:expr ),*) => { concat!("???", $( $l ),*) }; // "\x1B["
}

/// Derive a non-CSI msequence (static) struct.
/// derive_str_sequence!("Clear the entire screen.", All, "2J");
#[allow(unused_macros)]
macro_rules! derive_static_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Debug)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, $value)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &'static [u8] { $value.as_bytes() }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &'static str { $value }
        }
    };
}

#[allow(unused_macros)]
macro_rules! derive_const_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        pub const $name: &str = $value;

        // use std::fmt; 
        // impl fmt::Display for $name {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, $value)
        //     }
        // }

        // impl AsRef<[u8]> for $name {
        //     fn as_ref(&self) -> &'static [u8] { $value.as_bytes() }
        // }

        // impl AsRef<str> for $name {
        //     fn as_ref(&self) -> &'static str { $value }
        // }
    };
}
/// duplicated mtermion macros:
/// termion's Create a CSI-introduced sequence.
#[allow(unused_macros)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// termion's Derive a CSI sequence struct.
/// derive_csi_sequence!("Change the cursor style to blinking block", BlinkingBlock, "\x31 q");
#[allow(unused_macros)]
macro_rules! derive_csi_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, csi!($value))
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &'static [u8] { csi!($value).as_bytes() }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &'static str { csi!($value) }
        }
    };
}

/// derive_fn!("Get user name.", ACCT, "get_current_username()");
#[allow(unused_macros)]
macro_rules! derive_fn {
    ($doc:expr, $name:ident, $value:stmt, $type:ty) => {
        #[doc = $doc]
        pub fn $name() -> $type {
            $value
        }

        // impl fmt::Display for $name {
        //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //         write!(f, format!("fn {}{}{}", $doc, $name, $value))
        //     }
        // }

        //<fn()>? closure?
        // impl AsRef<[u8]> for $name {
        //     fn as_ref(&self) -> &'static [u8] { csi!($value).as_bytes() }
        // }

        // impl AsRef<str> for $name {
        //     fn as_ref(&self) -> &'static str { csi!($value) }
        // }
    };
}
// https://stackoverflow.com/questions/51344951/how-do-you-unwrap-a-result-on-ok-or-return-from-the-function-on-err
//  let res = unwrap_or_return!(callable(&mut param));
#[allow(unused_macros)]
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    }
}

