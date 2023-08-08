// main.rs MdClipboard

// extern crate clipmdplus
// extern crate clipmdpluslib
// extern crate clipmdplusmacro

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
