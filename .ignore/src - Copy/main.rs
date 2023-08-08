// main.rs MdClipboard

// #[macro_use]
// extern crate anyhow; (no - static types preferred)
// extern crate blob; (doesn't implement ToSql trait)
// extern crate derive_name;
// extern crate derive_more;
// extern crate egui;
// extern crate eframe; (for egui)
// extern crate serde;
// extern crate serde_derive;
// extern crate serde_enum_str; (buggy)
// extern crate serde_json;
// extern crate serde-error (no - uses anyhow::result)
// extern crate thiserror; (unit errors)

// #[macro_use]
// pub mod macros;
// pub use self::macros::*;

pub mod clip_util;
pub use self::clip_util::*;

pub mod clip_form;
pub use self::clip_form::*;

pub mod stdmd;
pub use self::stdmd::*;

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    println!("Hello, world!");
    let mut clip_form = clip_form::ClipForm::default();
    let mut ui_type = stdmd::types::UiType::EguiNative;
    let _clip_form_result = clip_form.run(&mut ui_type);
    Ok(())
}

// Note on error handling:
// I'm using thiserror but there are other popular crates:
// •	error-chain
// •	failure
// •	quick-error
// •	Anyhow
// •	SNAFU
