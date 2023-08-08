// #[macro_use]
// mod macros;
// pub mod macros;
// pub use self::macros::*;
// !------------------------------------------------------------
extern crate clipmdplusmacro;
pub use clipmdplusmacro::*;
// !------------------------------------------------------------
pub mod derive_name;
pub use self::derive_name::*;
//  include!(".\\macros\\derive_name.rs");
// !------------------------------------------------------------
